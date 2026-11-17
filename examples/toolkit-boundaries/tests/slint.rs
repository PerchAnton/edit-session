use edit_session::{EditSession, FinishError};
use slint::{Model, SharedString, VecModel};
#[test]
fn shared_string_preview_does_not_mutate_canonical_model() {
    let model = VecModel::from(vec![SharedString::from("original")]);
    let mut draft = EditSession::new(model.row_data(0).unwrap(), (7_u64, 0_u64));
    let mut proposal = draft.preview().clone();
    proposal.push_str(" edited");
    draft.propose::<()>(Ok(proposal)).unwrap();
    assert_eq!(model.row_data(0).unwrap(), "original");
    draft.cancel();
    assert_eq!(model.row_data(0).unwrap(), "original");
}
#[test]
fn same_row_number_does_not_identify_replacement_record() {
    let model = VecModel::from(vec![SharedString::from("original")]);
    let mut draft = EditSession::new(model.row_data(0).unwrap(), (7_u64, 0_u64));
    draft.propose::<()>(Ok("edited".into())).unwrap();
    // A replacement at row zero can have identical text; its stable ID differs.
    model.set_row_data(0, "original".into());
    assert_eq!(
        draft.finish(&model.row_data(0).unwrap(), &(8, 0)),
        Err(FinishError::ContextChanged)
    );
    assert_eq!(model.row_data(0).unwrap(), "original");
}
#[test]
fn accepted_change_updates_model_after_validation() {
    let model = VecModel::from(vec![SharedString::from("original")]);
    let mut draft = EditSession::new(model.row_data(0).unwrap(), (7_u64, 0_u64));
    draft.propose::<()>(Ok("edited".into())).unwrap();
    let c = draft
        .finish(&model.row_data(0).unwrap(), &(7, 0))
        .unwrap()
        .unwrap();
    model.set_row_data(0, c.after);
    assert_eq!(model.row_data(0).unwrap(), "edited");
}
