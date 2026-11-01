//! Consumer-facing compatibility checks; keep these on the MSRV and in the crate.
use edit_session::{Change, EditSession, FinishError};

// Access/proposal/cancel must remain usable without equality or Clone bounds.
fn replace_and_cancel<T, C>(mut draft: EditSession<T, C>, value: T) {
    let _: &T = draft.original();
    let _: &T = draft.preview();
    let _: &C = draft.context();
    let _: bool = draft.is_valid();
    assert_eq!(draft.propose::<()>(Ok(value)), Ok(()));
    draft.cancel();
}

#[test]
fn access_and_cancel_do_not_require_equality() {
    #[derive(Clone)]
    struct Value;
    struct Context;
    replace_and_cancel(EditSession::new(Value, Context), Value);
}

#[test]
fn finish_does_not_require_a_cloneable_or_debug_context() {
    #[derive(PartialEq)]
    struct Context(u64);
    let mut draft = EditSession::new(10, Context(7));
    assert_eq!(draft.propose::<()>(Ok(12)), Ok(()));
    let result = draft.finish(&10, &Context(7));
    assert_eq!(
        result,
        Ok(Some(Change {
            before: 10,
            after: 12
        }))
    );
}

#[test]
fn simultaneous_conflicts_have_a_defined_priority() {
    let mut draft = EditSession::new(10, ("document-a", 7));
    assert_eq!(draft.propose::<()>(Err(())), Err(()));
    assert_eq!(
        draft.check(&11, &("document-b", 8)),
        Err(FinishError::ContextChanged)
    );
    assert_eq!(
        draft.check(&11, &("document-a", 7)),
        Err(FinishError::ValueChanged)
    );
    assert_eq!(draft.check_live(&10, &("document-a", 7)), Ok(()));
    assert_eq!(
        draft.check(&10, &("document-a", 7)),
        Err(FinishError::InvalidProposal)
    );
    assert_eq!(
        draft.finish(&11, &("document-b", 8)),
        Err(FinishError::ContextChanged)
    );
}
