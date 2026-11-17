use edit_session::{EditSession, FinishError};
use futures::{StreamExt, executor::block_on};
use iced_runtime::{Action, Task, task};
fn receive(task: Task<EditSession<u32, u64>>) -> EditSession<u32, u64> {
    let mut stream = task::into_stream(task).unwrap();
    match block_on(stream.next()).unwrap() {
        Action::Output(session) => session,
        _ => panic!("unexpected runtime action"),
    }
}
#[test]
fn queued_confirmation_checks_at_delivery_not_at_scheduling() {
    let mut draft = EditSession::new(10, 0_u64);
    draft.propose::<()>(Ok(20)).unwrap();
    let queued = Task::done(draft);
    let live = (10, 1); // A refresh occurred before the task's message is handled.
    assert_eq!(
        receive(queued).finish(&live.0, &live.1),
        Err(FinishError::ContextChanged)
    );
}
#[test]
fn unchanged_confirmation_has_no_document_command() {
    let queued = Task::done(EditSession::new(10, 0_u64));
    assert_eq!(receive(queued).finish(&10, &0), Ok(None));
}
#[test]
fn changed_confirmation_commits_once_on_host_path() {
    let mut draft = EditSession::new(10, 0_u64);
    draft.propose::<()>(Ok(20)).unwrap();
    let c = receive(Task::done(draft)).finish(&10, &0).unwrap().unwrap();
    assert_eq!((c.before, c.after), (10, 20));
}
