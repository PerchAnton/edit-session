//! A synchronous host-owned command boundary: retain the draft until the command succeeds.
//! This small model demonstrates recovery; it is not a PhotoCraft/PdfCraft UI regression.
use edit_session::{EditSession, FinishError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Context {
    document: u64,
    target: u64,
    revision: u64,
}

#[derive(Debug, PartialEq, Eq)]
enum ConfirmError {
    Closed,
    Draft(FinishError),
    PermissionDenied,
    RevisionExhausted,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Applied,
    Unchanged,
}

struct Host {
    value: String,
    context: Context,
    allowed: bool,
    undo: Vec<String>,
    redo: Vec<String>,
}

impl Host {
    fn new() -> Self {
        Self {
            value: "original".into(),
            context: Context {
                document: 7,
                target: 2,
                revision: 42,
            },
            allowed: true,
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    fn apply(&mut self, proposed: &str) -> Result<(), ConfirmError> {
        // All fallible host checks precede domain/history writes. Real command engines
        // must provide the same atomic rejection guarantee, or their own rollback.
        if !self.allowed {
            return Err(ConfirmError::PermissionDenied);
        }
        let next_revision = self
            .context
            .revision
            .checked_add(1)
            .ok_or(ConfirmError::RevisionExhausted)?;
        self.undo.push(self.value.clone());
        self.value = proposed.to_owned();
        self.redo.clear();
        self.context.revision = next_revision;
        Ok(())
    }
}

struct Dialog {
    // Keep the raw field too: an invalid proposal retains only the last valid preview.
    text: String,
    draft: Option<EditSession<String, Context>>,
}

impl Dialog {
    fn open(host: &Host) -> Self {
        Self {
            text: host.value.clone(),
            draft: Some(EditSession::new(host.value.clone(), host.context)),
        }
    }

    fn set_text(&mut self, text: &str) -> Result<(), &'static str> {
        let draft = self.draft.as_mut().ok_or("dialog is closed")?;
        self.text = text.to_owned();
        draft.propose(if text.trim().is_empty() {
            Err("enter a title")
        } else {
            Ok(self.text.clone())
        })
    }

    fn confirm(&mut self, host: &mut Host) -> Result<Outcome, ConfirmError> {
        // This whole method runs on the host's serialized edit path, without an await
        // or a callback that can change the target between check and command dispatch.
        let draft = self.draft.as_ref().ok_or(ConfirmError::Closed)?;
        draft
            .check(&host.value, &host.context)
            .map_err(ConfirmError::Draft)?;
        let outcome = if draft.preview() == draft.original() {
            Outcome::Unchanged // Do not dispatch a command or clear Redo for a no-op.
        } else {
            // Borrow the proposal: finish() would consume it before this fallible call.
            host.apply(draft.preview())?;
            Outcome::Applied
        };
        self.cancel(); // Close only after a successful command or a checked no-op.
        Ok(outcome)
    }

    fn cancel(&mut self) {
        if let Some(draft) = self.draft.take() {
            draft.cancel();
        }
    }
}

fn main() -> Result<(), ConfirmError> {
    let mut host = Host::new();
    let mut dialog = Dialog::open(&host);
    assert_eq!(dialog.set_text("edited title"), Ok(()));
    host.allowed = false;
    assert_eq!(
        dialog.confirm(&mut host),
        Err(ConfirmError::PermissionDenied)
    );
    assert_eq!(dialog.text, "edited title");
    assert!(dialog.draft.is_some());
    host.allowed = true;
    assert_eq!(dialog.confirm(&mut host)?, Outcome::Applied);
    assert_eq!(host.value, "edited title");
    assert_eq!(host.undo, ["original"]);
    assert_eq!(dialog.confirm(&mut host), Err(ConfirmError::Closed));
    println!("Denied command retained the draft; retry applied once and closed the dialog.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn denied_edit() -> (Host, Dialog) {
        let mut host = Host::new();
        host.redo.push("future".into());
        let mut dialog = Dialog::open(&host);
        assert_eq!(dialog.set_text("edited"), Ok(()));
        host.allowed = false;
        assert_eq!(
            dialog.confirm(&mut host),
            Err(ConfirmError::PermissionDenied)
        );
        (host, dialog)
    }

    #[test]
    fn rejection_preserves_fields_document_revision_and_history_then_retry_commits_once() {
        let (mut host, mut dialog) = denied_edit();
        assert_eq!(host.value, "original");
        assert_eq!(host.context.revision, 42);
        assert!(host.undo.is_empty());
        assert_eq!(host.redo, ["future"]);
        assert_eq!(dialog.text, "edited");
        assert!(dialog.draft.is_some());
        host.allowed = true;
        assert_eq!(dialog.confirm(&mut host), Ok(Outcome::Applied));
        assert_eq!(host.value, "edited");
        assert_eq!(host.context.revision, 43);
        assert_eq!(host.undo, ["original"]);
        assert!(host.redo.is_empty());
        assert!(dialog.draft.is_none());
    }

    #[test]
    fn repeated_confirm_cannot_duplicate_the_command() {
        let (mut host, mut dialog) = denied_edit();
        host.allowed = true;
        assert_eq!(dialog.confirm(&mut host), Ok(Outcome::Applied));
        assert_eq!(dialog.confirm(&mut host), Err(ConfirmError::Closed));
        assert_eq!(host.undo, ["original"]);
        assert_eq!(host.context.revision, 43);
    }

    #[test]
    fn invalid_correction_keeps_raw_input_and_does_not_apply_last_valid_preview() {
        let (mut host, mut dialog) = denied_edit();
        host.allowed = true;
        assert!(dialog.set_text("  ").is_err());
        assert_eq!(
            dialog.confirm(&mut host),
            Err(ConfirmError::Draft(FinishError::InvalidProposal))
        );
        assert_eq!(dialog.text, "  ");
        assert_eq!(host.value, "original");
        assert!(host.undo.is_empty());
        assert_eq!(dialog.set_text("corrected"), Ok(()));
        assert_eq!(dialog.confirm(&mut host), Ok(Outcome::Applied));
        assert_eq!(host.value, "corrected");
    }

    #[test]
    fn retry_never_retargets_a_different_document_or_target() {
        for change_document in [true, false] {
            let (mut host, mut dialog) = denied_edit();
            host.allowed = true;
            if change_document {
                host.context.document += 1;
            } else {
                host.context.target += 1;
            }
            assert_eq!(
                dialog.confirm(&mut host),
                Err(ConfirmError::Draft(FinishError::ContextChanged))
            );
            assert_eq!(host.value, "original");
            assert!(host.undo.is_empty());
            assert_eq!(host.redo, ["future"]);
            assert_eq!(dialog.text, "edited");
        }
    }

    #[test]
    fn edit_then_undo_before_retry_is_stale_even_at_the_original_value() {
        let (mut host, mut dialog) = denied_edit();
        host.allowed = true;
        host.context.revision += 2; // A host edit and Undo, back to "original".
        assert_eq!(
            dialog.confirm(&mut host),
            Err(ConfirmError::Draft(FinishError::ContextChanged))
        );
        assert_eq!(host.value, "original");
        assert!(host.undo.is_empty());
        assert_eq!(host.redo, ["future"]);
    }

    #[test]
    fn unversioned_host_value_change_still_blocks_retry() {
        let (mut host, mut dialog) = denied_edit();
        host.allowed = true;
        host.value = "newer value".into();
        assert_eq!(
            dialog.confirm(&mut host),
            Err(ConfirmError::Draft(FinishError::ValueChanged))
        );
        assert_eq!(host.value, "newer value");
        assert!(host.undo.is_empty());
    }

    #[test]
    fn noop_skips_the_denied_command_and_preserves_redo() {
        let (mut host, mut dialog) = denied_edit();
        assert_eq!(dialog.set_text("original"), Ok(()));
        assert_eq!(dialog.confirm(&mut host), Ok(Outcome::Unchanged));
        assert!(dialog.draft.is_none());
        assert_eq!(host.context.revision, 42);
        assert!(host.undo.is_empty());
        assert_eq!(host.redo, ["future"]);
    }

    #[test]
    fn cancel_after_rejection_discards_only_the_local_draft() {
        let (mut host, mut dialog) = denied_edit();
        dialog.cancel();
        assert_eq!(dialog.confirm(&mut host), Err(ConfirmError::Closed));
        assert_eq!(host.value, "original");
        assert_eq!(host.context.revision, 42);
        assert!(host.undo.is_empty());
        assert_eq!(host.redo, ["future"]);
    }
}
