//! Request identity and draft context protect different boundaries.
//! This model performs no I/O and is not an upstream application reproduction.
use edit_session::{EditSession, FinishError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Context {
    document: u64,
    revision: u64,
}
#[derive(Debug, PartialEq, Eq)]
enum State {
    Closed,
    Planning(u64),
    Ready(String),
    Failed(String),
}
struct Panel {
    generation: u64,
    state: State,
    draft: Option<EditSession<String, Context>>,
}
impl Panel {
    fn new() -> Self {
        Self {
            generation: 0,
            state: State::Closed,
            draft: None,
        }
    }
    fn start(&mut self, value: &str, context: Context) -> Result<u64, &'static str> {
        // Exhaustion fails before changing the current request or draft.
        let ticket = self
            .generation
            .checked_add(1)
            .ok_or("generation exhausted")?;
        self.cancel();
        self.generation = ticket;
        self.state = State::Planning(ticket);
        self.draft = Some(EditSession::new(value.to_owned(), context));
        Ok(ticket)
    }
    fn receive(&mut self, ticket: u64, result: Result<String, String>) -> bool {
        // Reject stale success AND failure before proposing into a replacement draft.
        if self.state != State::Planning(ticket) {
            return false;
        }
        self.state = match result {
            Ok(value) => {
                if let Some(draft) = &mut self.draft {
                    let _: Result<(), ()> = draft.propose(Ok(value.clone()));
                }
                State::Ready(value)
            }
            Err(error) => State::Failed(error),
        };
        true
    }
    fn confirm(&mut self, live: &mut String, context: &Context) -> Result<bool, FinishError> {
        if !matches!(self.state, State::Ready(_)) {
            return Err(FinishError::InvalidProposal);
        }
        let Some(draft) = self.draft.as_ref() else {
            return Err(FinishError::InvalidProposal);
        };
        draft.check(live, context)?;
        let changed = draft.preview() != live;
        // A real host dispatches its atomic command here, on the serialized edit path.
        // Fallible commands should use retry_command.rs and retain the draft on error.
        if changed {
            *live = draft.preview().clone();
        }
        self.cancel();
        Ok(changed)
    }
    fn cancel(&mut self) {
        if let Some(draft) = self.draft.take() {
            draft.cancel();
        }
        self.state = State::Closed;
    }
}
fn main() -> Result<(), &'static str> {
    let context = Context {
        document: 1,
        revision: 1,
    };
    let mut panel = Panel::new();
    let old = panel.start("original", context)?;
    panel.cancel();
    let current = panel.start("original", context)?;
    assert!(!panel.receive(old, Ok("stale".into())));
    assert!(panel.receive(current, Ok("current".into())));
    let mut live = "original".into();
    assert_eq!(panel.confirm(&mut live, &context), Ok(true));
    assert_eq!(live, "current");
    println!("The current request committed once after document context validation.");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn context() -> Context {
        Context {
            document: 1,
            revision: 7,
        }
    }
    #[test]
    fn stale_success_and_failure_cannot_replace_new_request() -> Result<(), &'static str> {
        for result in [Ok("old plan".into()), Err("old error".into())] {
            let mut p = Panel::new();
            let old = p.start("original", context())?;
            p.cancel();
            let next = p.start("original", context())?;
            assert!(!p.receive(old, result));
            assert_eq!(p.state, State::Planning(next));
            assert!(p.receive(next, Ok("new plan".into())));
            let mut live = "original".into();
            assert_eq!(p.confirm(&mut live, &context()), Ok(true));
            assert_eq!(live, "new plan");
            assert_eq!(p.state, State::Closed);
        }
        Ok(())
    }
    #[test]
    fn closing_without_reopening_discards_worker_result() -> Result<(), &'static str> {
        let mut p = Panel::new();
        let ticket = p.start("original", context())?;
        p.cancel();
        assert!(!p.receive(ticket, Ok("late".into())));
        assert_eq!(p.state, State::Closed);
        Ok(())
    }
    #[test]
    fn same_value_on_another_document_is_stale() -> Result<(), &'static str> {
        let mut p = Panel::new();
        let ticket = p.start("original", context())?;
        assert!(p.receive(ticket, Ok("proposal".into())));
        let mut live = "original".into();
        let other = Context {
            document: 2,
            ..context()
        };
        assert_eq!(
            p.confirm(&mut live, &other),
            Err(FinishError::ContextChanged)
        );
        assert_eq!(live, "original");
        Ok(())
    }
    #[test]
    fn undo_back_to_original_value_does_not_restore_revision() -> Result<(), &'static str> {
        let mut p = Panel::new();
        let ticket = p.start("original", context())?;
        assert!(p.receive(ticket, Ok("proposal".into())));
        let mut live = "original".into();
        let after_undo = Context {
            revision: 9,
            ..context()
        };
        assert_eq!(
            p.confirm(&mut live, &after_undo),
            Err(FinishError::ContextChanged)
        );
        assert_eq!(live, "original");
        Ok(())
    }
    #[test]
    fn matching_ticket_does_not_allow_confirmation_after_failed_plan() -> Result<(), &'static str> {
        let mut p = Panel::new();
        let ticket = p.start("original", context())?;
        assert!(p.receive(ticket, Err("bad archive".into())));
        let mut live = "original".into();
        assert_eq!(
            p.confirm(&mut live, &context()),
            Err(FinishError::InvalidProposal)
        );
        assert_eq!(live, "original");
        Ok(())
    }
    #[test]
    fn duplicate_result_and_duplicate_confirmation_are_rejected() -> Result<(), &'static str> {
        let mut p = Panel::new();
        let ticket = p.start("original", context())?;
        assert!(p.receive(ticket, Ok("proposal".into())));
        assert!(!p.receive(ticket, Ok("duplicate".into())));
        let mut live = "original".into();
        assert_eq!(p.confirm(&mut live, &context()), Ok(true));
        assert_eq!(
            p.confirm(&mut live, &context()),
            Err(FinishError::InvalidProposal)
        );
        assert_eq!(live, "proposal");
        Ok(())
    }
    #[test]
    fn no_op_produces_no_change() -> Result<(), &'static str> {
        let mut p = Panel::new();
        let ticket = p.start("original", context())?;
        p.receive(ticket, Ok("original".into()));
        let mut live = "original".into();
        assert_eq!(p.confirm(&mut live, &context()), Ok(false));
        Ok(())
    }
    #[test]
    fn generation_exhaustion_preserves_current_request() -> Result<(), &'static str> {
        let mut p = Panel::new();
        let ticket = p.start("original", context())?;
        p.generation = u64::MAX;
        assert_eq!(p.start("other", context()), Err("generation exhausted"));
        assert_eq!(p.state, State::Planning(ticket));
        Ok(())
    }
}
