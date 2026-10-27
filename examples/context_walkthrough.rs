//! Why an edit followed by Undo still invalidates an older draft.
//! An original synchronous teaching model, not an application integration test.
use edit_session::{EditSession, FinishError};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Context {
    document: u64,
    target: u64,
    revision: u64,
}

struct Row {
    action: &'static str,
    live: u8,
    revision: u64,
    preview: u8,
    value_matches_original: bool,
    check: &'static str,
}

fn observe(
    action: &'static str,
    live: u8,
    context: Context,
    draft: &EditSession<u8, Context>,
) -> Row {
    let check = match draft.check(&live, &context) {
        Ok(()) => "Ready",
        Err(FinishError::ContextChanged) => "ContextChanged",
        Err(FinishError::ValueChanged) => "ValueChanged",
        Err(FinishError::InvalidProposal) => "InvalidProposal",
    };
    Row {
        action,
        live,
        revision: context.revision,
        preview: *draft.preview(),
        value_matches_original: live == *draft.original(),
        check,
    }
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let json = match args.as_slice() {
        [] => false,
        [flag] if flag == "--json" => true,
        _ => {
            eprintln!("Usage: context_walkthrough [--json]");
            std::process::exit(2);
        }
    };
    let mut live = 100_u8;
    let mut context = Context {
        document: 7,
        target: 2,
        revision: 42,
    };
    let mut draft = EditSession::new(live, context);
    assert_eq!(draft.propose::<&str>(Ok(75)), Ok(()));
    let mut rows = vec![observe("Open draft", live, context, &draft)];
    assert_eq!(draft.check(&live, &context), Ok(()));

    // The host owns both the external edit and its monotonic revision.
    live = 120;
    context.revision += 1;
    rows.push(observe("Host edits", live, context, &draft));
    assert_eq!(
        draft.check(&live, &context),
        Err(FinishError::ContextChanged)
    );

    // Undo restores the VALUE, but must advance the host's revision again.
    live = 100;
    context.revision += 1;
    rows.push(observe("Host undoes", live, context, &draft));
    assert_eq!(live, *draft.original());
    assert_eq!(
        draft.check(&live, &context),
        Err(FinishError::ContextChanged)
    );

    // Reset changes the preview, never the captured document/target/revision.
    draft.reset();
    rows.push(observe("Reset draft", live, context, &draft));
    assert_eq!(*draft.preview(), live);
    assert_eq!(draft.context().revision, 42);
    assert_eq!(
        draft.finish(&live, &context),
        Err(FinishError::ContextChanged)
    );

    if json {
        println!("{{\"rows\":[");
        for (i, row) in rows.iter().enumerate() {
            println!(
                "{{\"action\":\"{}\",\"live\":{},\"revision\":{},\"preview\":{},\"value_matches_original\":{},\"check\":\"{}\"}}{}",
                row.action,
                row.live,
                row.revision,
                row.preview,
                row.value_matches_original,
                row.check,
                if i + 1 == rows.len() { "" } else { "," }
            );
        }
        println!("],\"finish\":\"ContextChanged\"}}");
    } else {
        println!("Action        | Live | Revision | Preview | Value equals original | check()");
        for row in &rows {
            println!(
                "{:<13} | {:>4} | {:>8} | {:>7} | {:<21} | {}",
                row.action,
                row.live,
                row.revision,
                row.preview,
                row.value_matches_original,
                row.check
            );
        }
        println!("finish(): ContextChanged. The host must not apply the old draft.");
    }
}
