use edit_session::{EditSession, FinishError};
use vizia_id::*;
#[derive(Copy, Clone, PartialEq)]
struct Target(u64);
impl_generational_id!(Target);
#[test]
fn reused_slot_is_a_different_edit_target() {
    let mut ids = IdManager::<Target>::new();
    let old = ids.create();
    let mut draft = EditSession::new(10, (old, 0_u64));
    draft.propose::<()>(Ok(20)).unwrap();
    assert!(ids.destroy(old));
    for _ in 0..4095 {
        let id = ids.create();
        assert!(ids.destroy(id));
    }
    let replacement = ids.create();
    assert_eq!(old.index(), replacement.index());
    assert_ne!(old.generation(), replacement.generation());
    assert!(!ids.is_alive(old));
    assert!(ids.is_alive(replacement));
    assert_eq!(
        draft.finish(&10, &(replacement, 0)),
        Err(FinishError::ContextChanged)
    );
}
#[test]
fn undo_to_same_value_still_changes_document_revision() {
    let mut ids = IdManager::<Target>::new();
    let id = ids.create();
    let mut draft = EditSession::new(10, (id, 4_u64));
    draft.propose::<()>(Ok(20)).unwrap();
    assert_eq!(
        draft.finish(&10, &(id, 6)),
        Err(FinishError::ContextChanged)
    );
}
