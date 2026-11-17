use edit_session::{EditSession, FinishError};
use sycamore_reactive::{create_root, create_signal};
#[test]
fn draft_signal_updates_do_not_publish_and_cancel_discards() {
    let root = create_root(|| {
        let live = create_signal((10, 0_u64));
        let draft = create_signal(Some(EditSession::new(10, 0_u64)));
        draft.update(|s| s.as_mut().unwrap().propose::<()>(Ok(20)).unwrap());
        assert_eq!(draft.with(|s| *s.as_ref().unwrap().preview()), 20);
        assert_eq!(live.get(), (10, 0));
        draft.update(|s| s.take().unwrap().cancel());
        assert!(draft.with(Option::is_none));
        assert_eq!(live.get(), (10, 0));
    });
    root.dispose();
}
#[test]
fn latest_signal_state_is_checked_inside_serial_update() {
    let root = create_root(|| {
        let live = create_signal((10, 0_u64));
        let draft = create_signal(Some(EditSession::new(10, 0_u64)));
        draft.update(|s| s.as_mut().unwrap().propose::<()>(Ok(20)).unwrap());
        live.update(|s| s.1 += 1);
        let s = draft.update(|s| s.take().unwrap());
        assert_eq!(
            live.update(|x| s.finish(&x.0, &x.1)),
            Err(FinishError::ContextChanged)
        );
        let mut fresh = EditSession::new(10, 1);
        fresh.propose::<()>(Ok(30)).unwrap();
        live.update(|x| {
            let c = fresh.finish(&x.0, &x.1).unwrap().unwrap();
            x.0 = c.after;
            x.1 += 1;
        });
        assert_eq!(live.get(), (30, 2));
    });
    root.dispose();
}
