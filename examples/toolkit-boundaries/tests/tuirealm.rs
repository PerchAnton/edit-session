use edit_session::{EditSession, FinishError};
use tuirealm::{
    command::{Cmd, CmdResult},
    state::{State, StateValue},
};
fn propose(draft: &mut EditSession<u16, u64>, state: State) -> Result<(), &'static str> {
    let value = match state {
        State::Single(StateValue::String(s)) => s.parse::<u16>().map_err(|_| "invalid number"),
        _ => Err("wrong state type"),
    };
    draft.propose(value)
}
fn confirm(
    draft: EditSession<u16, u64>,
    live: u16,
    revision: u64,
) -> Result<CmdResult, FinishError> {
    draft.finish(&live, &revision).map(|c| {
        c.map_or(CmdResult::NoChange, |c| {
            CmdResult::Submit(State::Single(StateValue::U16(c.after)))
        })
    })
}
#[test]
fn invalid_input_keeps_preview_but_cannot_submit() {
    let mut d = EditSession::new(10, 0);
    propose(&mut d, State::Single(StateValue::String("20".into()))).unwrap();
    assert_eq!(
        propose(&mut d, State::Single(StateValue::String("-".into()))),
        Err("invalid number")
    );
    assert_eq!(*d.preview(), 20);
    assert_eq!(confirm(d, 10, 0), Err(FinishError::InvalidProposal));
}
#[test]
fn field_commands_map_to_one_submit_or_local_cancel() {
    for cmd in [Cmd::Submit, Cmd::Cancel] {
        let mut d = EditSession::new(10, 0);
        propose(&mut d, State::Single(StateValue::String("20".into()))).unwrap();
        match cmd {
            Cmd::Submit => assert_eq!(
                confirm(d, 10, 0),
                Ok(CmdResult::Submit(State::Single(StateValue::U16(20))))
            ),
            Cmd::Cancel => d.cancel(),
            _ => unreachable!(),
        }
    }
    assert_eq!(
        confirm(EditSession::new(10, 0), 10, 0),
        Ok(CmdResult::NoChange)
    );
}
#[test]
fn component_snapshot_does_not_authorize_stale_submit() {
    let mut d = EditSession::new(10, 0);
    propose(&mut d, State::Single(StateValue::String("20".into()))).unwrap();
    assert_eq!(confirm(d, 10, 1), Err(FinishError::ContextChanged));
}
