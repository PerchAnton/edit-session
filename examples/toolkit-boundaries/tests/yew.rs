use edit_session::{EditSession, FinishError};
use std::{cell::RefCell, rc::Rc};
use yew::Callback;
#[derive(Default)]
struct Host {
    value: u32,
    revision: u64,
    writes: usize,
}
fn callback(
    host: Rc<RefCell<Host>>,
    draft: Rc<RefCell<Option<EditSession<u32, u64>>>>,
    results: Rc<RefCell<Vec<Result<bool, FinishError>>>>,
) -> Callback<()> {
    Callback::from(move |()| {
        if let Some(session) = draft.borrow_mut().take() {
            let mut h = host.borrow_mut();
            let outcome = session.finish(&h.value, &h.revision).map(|change| {
                if let Some(c) = change {
                    h.value = c.after;
                    h.revision += 1;
                    h.writes += 1;
                    true
                } else {
                    false
                }
            });
            results.borrow_mut().push(outcome);
        }
    })
}
#[test]
fn duplicate_callback_does_not_commit_twice() {
    let h = Rc::new(RefCell::new(Host::default()));
    let mut s = EditSession::new(0, 0);
    s.propose::<()>(Ok(5)).unwrap();
    let d = Rc::new(RefCell::new(Some(s)));
    let r = Rc::new(RefCell::new(Vec::new()));
    let cb = callback(h.clone(), d, r.clone());
    cb.emit(());
    cb.emit(());
    assert_eq!(h.borrow().value, 5);
    assert_eq!(h.borrow().writes, 1);
    assert_eq!(*r.borrow(), [Ok(true)]);
}
#[test]
fn callback_reads_current_host_revision() {
    let h = Rc::new(RefCell::new(Host::default()));
    let mut s = EditSession::new(0, 0);
    s.propose::<()>(Ok(5)).unwrap();
    let d = Rc::new(RefCell::new(Some(s)));
    let r = Rc::new(RefCell::new(Vec::new()));
    let cb = callback(h.clone(), d, r.clone());
    h.borrow_mut().revision = 1;
    cb.emit(());
    assert_eq!(*r.borrow(), [Err(FinishError::ContextChanged)]);
    assert_eq!(h.borrow().writes, 0);
}
