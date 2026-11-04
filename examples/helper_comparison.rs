//! Standalone synchronous teaching model. No upstream bug or adoption claim.
//! Run with cargo test --locked --example helper_comparison.
use edit_session::{EditSession, FinishError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Context {
    document: u64,
    object: u64,
    revision: u64,
}

// BEGIN local lifecycle: count separately from common host and tests.
struct LocalDraft {
    original: i32,
    preview: i32,
    context: Context,
    valid: bool,
}
impl LocalDraft {
    fn new(value: i32, context: Context) -> Self {
        Self {
            original: value,
            preview: value,
            context,
            valid: true,
        }
    }
    fn propose(&mut self, value: Result<i32, &'static str>) {
        self.valid = value.is_ok();
        if let Ok(value) = value {
            self.preview = value;
        }
    }
    fn finish(self, live: i32, context: Context) -> Result<Option<i32>, &'static str> {
        if context != self.context {
            return Err("context changed");
        }
        if live != self.original {
            return Err("value changed");
        }
        if !self.valid {
            return Err("invalid proposal");
        }
        Ok((self.preview != self.original).then_some(self.preview))
    }
    fn cancel(self) {}
}
// END local lifecycle.

// BEGIN library adapter: same application-facing result, no local lifecycle.
struct LibraryDraft(EditSession<i32, Context>);
impl LibraryDraft {
    fn new(value: i32, context: Context) -> Self {
        Self(EditSession::new(value, context))
    }
    fn propose(&mut self, value: Result<i32, &'static str>) {
        let _ = self.0.propose(value);
    }
    fn finish(self, live: i32, context: Context) -> Result<Option<i32>, &'static str> {
        self.0
            .finish(&live, &context)
            .map(|v| v.map(|c| c.after))
            .map_err(|e| match e {
                FinishError::ContextChanged => "context changed",
                FinishError::ValueChanged => "value changed",
                FinishError::InvalidProposal => "invalid proposal",
            })
    }
    fn cancel(self) {
        self.0.cancel();
    }
}
// END library adapter.

// Shared host/test plumbing is required in either implementation.
enum Draft {
    Local(LocalDraft),
    Library(LibraryDraft),
}
impl Draft {
    fn open(library: bool, host: &Host) -> Self {
        if library {
            Self::Library(LibraryDraft::new(host.value, host.context))
        } else {
            Self::Local(LocalDraft::new(host.value, host.context))
        }
    }
    fn propose(&mut self, value: Result<i32, &'static str>) {
        match self {
            Self::Local(d) => d.propose(value),
            Self::Library(d) => d.propose(value),
        }
    }
    fn finish(self, host: &mut Host) -> Result<(), &'static str> {
        // No await, callback or intervening mutation between comparison and write.
        let proposal = match self {
            Self::Local(d) => d.finish(host.value, host.context),
            Self::Library(d) => d.finish(host.value, host.context),
        }?;
        if let Some(value) = proposal {
            host.apply(value);
        }
        Ok(())
    }
    fn cancel(self, _host: &mut Host) {
        match self {
            Self::Local(d) => d.cancel(),
            Self::Library(d) => d.cancel(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Host {
    value: i32,
    context: Context,
    commands: usize,
    redo: usize,
}
impl Host {
    fn new() -> Self {
        Self {
            value: 100,
            context: Context {
                document: 7,
                object: 2,
                revision: 42,
            },
            commands: 0,
            redo: 1,
        }
    }
    fn apply(&mut self, value: i32) {
        self.value = value;
        self.context.revision += 1; // Bounded synthetic sequence; production uses checked arithmetic.
        self.commands += 1;
        self.redo = 0;
    }
}

fn main() {
    for library in [false, true] {
        let label = if library {
            "edit-session (current checkout)"
        } else {
            "local helper"
        };
        let mut host = Host::new();
        let mut draft = Draft::open(library, &host);
        draft.propose(Ok(75));
        host.apply(120);
        draft.cancel(&mut host);
        println!(
            "{label}: Cancel after external edit -> live {}, revision {}, commands {}",
            host.value, host.context.revision, host.commands
        );
        let mut draft = Draft::open(library, &host);
        draft.propose(Ok(90));
        host.context.object = 3;
        println!(
            "{label}: same value, different object -> {:?}",
            draft.finish(&mut host)
        );
        let mut draft = Draft::open(library, &host);
        draft.propose(Ok(90));
        host.apply(150);
        host.apply(120);
        println!("{label}: edit then Undo -> {:?}", draft.finish(&mut host));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn each(test: impl Fn(bool)) {
        for library in [false, true] {
            test(library);
        }
    }
    #[test]
    fn cancel_preserves_external_edit_and_history() {
        each(|library| {
            let mut h = Host::new();
            let mut d = Draft::open(library, &h);
            d.propose(Ok(75));
            h.apply(120);
            let before = h.clone();
            d.cancel(&mut h);
            assert_eq!(h, before);
        });
    }
    #[test]
    fn same_value_different_object_is_rejected() {
        each(|library| {
            let mut h = Host::new();
            let mut d = Draft::open(library, &h);
            d.propose(Ok(75));
            h.context.object = 3;
            let before = h.clone();
            assert_eq!(d.finish(&mut h), Err("context changed"));
            assert_eq!(h, before);
        });
    }
    #[test]
    fn same_value_different_document_is_rejected() {
        each(|library| {
            let mut h = Host::new();
            let mut d = Draft::open(library, &h);
            d.propose(Ok(75));
            h.context.document = 8;
            let before = h.clone();
            assert_eq!(d.finish(&mut h), Err("context changed"));
            assert_eq!(h, before);
        });
    }
    #[test]
    fn edit_then_undo_is_still_stale() {
        each(|library| {
            let mut h = Host::new();
            let mut d = Draft::open(library, &h);
            d.propose(Ok(75));
            h.apply(120);
            h.apply(100);
            let before = h.clone();
            assert_eq!(d.finish(&mut h), Err("context changed"));
            assert_eq!(h, before);
        });
    }
    #[test]
    fn changed_value_without_revision_is_rejected() {
        each(|library| {
            let mut h = Host::new();
            let mut d = Draft::open(library, &h);
            d.propose(Ok(75));
            h.value = 120;
            let before = h.clone();
            assert_eq!(d.finish(&mut h), Err("value changed"));
            assert_eq!(h, before);
        });
    }
    #[test]
    fn invalid_input_cannot_commit_previous_valid_preview() {
        each(|library| {
            let mut h = Host::new();
            let mut d = Draft::open(library, &h);
            d.propose(Ok(75));
            d.propose(Err("not a number"));
            let before = h.clone();
            assert_eq!(d.finish(&mut h), Err("invalid proposal"));
            assert_eq!(h, before);
        });
    }
    #[test]
    fn no_op_preserves_redo() {
        each(|library| {
            let mut h = Host::new();
            let d = Draft::open(library, &h);
            let before = h.clone();
            assert_eq!(d.finish(&mut h), Ok(()));
            assert_eq!(h, before);
        });
    }
    #[test]
    fn fresh_draft_commits_once() {
        each(|library| {
            let mut h = Host::new();
            let mut d = Draft::open(library, &h);
            d.propose(Ok(75));
            assert_eq!(d.finish(&mut h), Ok(()));
            assert_eq!(
                (h.value, h.context.revision, h.commands, h.redo),
                (75, 43, 1, 0)
            );
        });
    }
}
