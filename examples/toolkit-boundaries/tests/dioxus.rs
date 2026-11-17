use dioxus_core::{Element, ScopeId, VirtualDom};
use dioxus_signals::{ReadableExt, Signal, WritableExt};
use edit_session::{EditSession, FinishError};
fn app() -> Element {
    dioxus_core::VNode::empty()
}
#[test]
fn local_signal_draft_cancel_and_late_confirmation() {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dom.in_scope(ScopeId::ROOT, || {
        let mut live = Signal::new(("original".to_string(), 0_u64));
        let mut draft = Signal::new(Some(EditSession::new(live.read().0.clone(), 0_u64)));
        draft.with_mut(|d| {
            d.as_mut()
                .unwrap()
                .propose::<()>(Ok("preview".into()))
                .unwrap()
        });
        assert_eq!(live.read().0, "original");
        draft.with_mut(|d| d.take().unwrap().cancel());
        assert_eq!(live.read().0, "original");
        draft.set(Some(EditSession::new(live.read().0.clone(), 0)));
        draft.with_mut(|d| {
            d.as_mut()
                .unwrap()
                .propose::<()>(Ok("edited".into()))
                .unwrap()
        });
        // A background refresh can leave the same value but advance the revision.
        live.with_mut(|x| x.1 += 1);
        let session = draft.with_mut(|d| d.take().unwrap());
        let result = live.with_mut(|x| session.finish(&x.0, &x.1));
        assert_eq!(result, Err(FinishError::ContextChanged));
        assert_eq!(live.read().0, "original");
    });
}
#[test]
fn confirm_consumes_non_clone_session_and_publishes_once() {
    let dom = VirtualDom::new(app);
    dom.in_scope(ScopeId::ROOT, || {
        let mut live = Signal::new((10, 0_u64));
        let mut draft = Signal::new(Some(EditSession::new(10, 0_u64)));
        draft.with_mut(|d| d.as_mut().unwrap().propose::<()>(Ok(20)).unwrap());
        let session = draft.with_mut(|d| d.take().unwrap());
        live.with_mut(|x| {
            let change = session.finish(&x.0, &x.1).unwrap().unwrap();
            x.0 = change.after;
            x.1 += 1;
        });
        assert_eq!(*live.read(), (20, 1));
        assert!(draft.read().is_none());
    });
}
