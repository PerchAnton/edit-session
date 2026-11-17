use edit_session::{EditSession, FinishError};
use ratatui_core::{buffer::Buffer, layout::Rect, style::Color};
use tachyonfx::{Duration, Interpolation, fx};
#[derive(Clone, Debug, PartialEq)]
struct Settings {
    color: Color,
    duration_ms: u32,
}
fn render(settings: &Settings, elapsed: u32) -> Buffer {
    let area = Rect::new(0, 0, 2, 1);
    let mut buf = Buffer::empty(area);
    for cell in &mut buf.content {
        cell.set_symbol("X");
        cell.set_fg(Color::Rgb(255, 255, 255));
    }
    // A preview render starts a separate effect. Do not clone a running timer as the document.
    let mut effect = fx::fade_to_fg(
        settings.color,
        (settings.duration_ms, Interpolation::Linear),
    );
    effect.process(Duration::from_millis(elapsed), &mut buf, area);
    buf
}
fn original() -> Settings {
    Settings {
        color: Color::Rgb(255, 0, 0),
        duration_ms: 100,
    }
}
#[test]
fn preview_uses_separate_effect_and_cancel_restores_canonical_frame() {
    let live = original();
    let baseline = render(&live, 100);
    let mut draft = EditSession::new(live.clone(), (7, 0));
    draft
        .propose::<()>(Ok(Settings {
            color: Color::Rgb(0, 0, 255),
            duration_ms: 100,
        }))
        .unwrap();
    let preview = render(draft.preview(), 100);
    assert_ne!(preview, baseline);
    assert_eq!(preview.content[0].fg, Color::Rgb(0, 0, 255));
    assert_eq!(render(&live, 100), baseline);
    draft.cancel();
    assert_eq!(render(&live, 100), baseline);
}
#[test]
fn stale_selected_effect_cannot_be_overwritten() {
    let live = original();
    let mut draft = EditSession::new(live.clone(), (7, 0));
    draft
        .propose::<()>(Ok(Settings {
            color: Color::Rgb(0, 0, 255),
            duration_ms: 50,
        }))
        .unwrap();
    assert_eq!(
        draft.finish(&live, &(8, 0)),
        Err(FinishError::ContextChanged)
    );
}
#[test]
fn confirm_changes_configuration_before_rebuilding_effect() {
    let live = original();
    let mut draft = EditSession::new(live.clone(), (7, 0));
    draft
        .propose::<()>(Ok(Settings {
            color: Color::Rgb(0, 0, 255),
            duration_ms: 50,
        }))
        .unwrap();
    let c = draft.finish(&live, &(7, 0)).unwrap().unwrap();
    assert_eq!(render(&c.after, 50).content[0].fg, Color::Rgb(0, 0, 255));
    assert_eq!(c.before, original());
}
