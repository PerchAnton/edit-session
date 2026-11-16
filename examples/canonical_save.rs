//! The host publishes the canonical value only after an atomic successful write.
use edit_session::EditSession;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Settings {
    increment: u16,
    icon: String,
}
#[derive(Debug)]
struct Host {
    saved: Settings,
    revision: u64,
    reject_write: bool,
    writes: usize,
}
impl Host {
    fn new() -> Self {
        Self {
            saved: Settings {
                increment: 10,
                icon: ">".into(),
            },
            revision: 0,
            reject_write: false,
            writes: 0,
        }
    }
    fn commit(&mut self, value: &Settings) -> Result<(), &'static str> {
        // This tiny example accepts one printable ASCII icon, not full terminal-width rules.
        if value.icon.len() != 1 || !value.icon.bytes().all(|b| b.is_ascii_graphic()) {
            return Err("invalid icon");
        }
        let canonical = Settings {
            increment: value.increment.clamp(1, 100),
            icon: value.icon.clone(),
        };
        if canonical == self.saved {
            return Ok(());
        }
        if self.reject_write {
            return Err("write failed");
        }
        let revision = self.revision.checked_add(1).ok_or("revision exhausted")?;
        // A real host must make storage and memory commit atomic or supply rollback.
        self.saved = canonical;
        self.revision = revision;
        self.writes += 1;
        Ok(())
    }
}
struct Form {
    raw_increment: String,
    raw_icon: String,
    saved_snapshot: Settings,
    draft: EditSession<Settings, u64>,
}
impl Form {
    fn open(host: &Host) -> Self {
        Self {
            raw_increment: host.saved.increment.to_string(),
            raw_icon: host.saved.icon.clone(),
            saved_snapshot: host.saved.clone(),
            draft: EditSession::new(host.saved.clone(), host.revision),
        }
    }
    fn edit(&mut self, number: &str, icon: &str) -> Result<(), &'static str> {
        self.raw_increment = number.into();
        self.raw_icon = icon.into();
        self.draft.propose(
            number
                .parse()
                .map(|increment| Settings {
                    increment,
                    icon: icon.into(),
                })
                .map_err(|_| "invalid number"),
        )
    }
    fn save(&mut self, host: &mut Host) -> Result<(), &'static str> {
        self.draft
            .check(&host.saved, &host.revision)
            .map_err(|_| "invalid or stale draft")?;
        host.commit(self.draft.preview())?;
        // Reload the accepted value. Rebuilding the draft after success is an explicit new session.
        *self = Self::open(host);
        Ok(())
    }
    fn cancel(self) {
        self.draft.cancel();
    }
}
fn main() -> Result<(), &'static str> {
    let mut host = Host::new();
    let mut form = Form::open(&host);
    form.edit("500", ">")?;
    form.save(&mut host)?;
    assert_eq!(form.raw_increment, "100");
    assert_eq!(form.saved_snapshot, host.saved);
    form.cancel();
    println!("500 was normalized to 100; the visible fields and saved snapshot both show 100.");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalization_reloads_fields_and_saved_snapshot() {
        let mut host = Host::new();
        let mut form = Form::open(&host);
        assert_eq!(form.edit("500", ">"), Ok(()));
        assert_eq!(form.save(&mut host), Ok(()));
        assert_eq!(form.raw_increment, "100");
        assert_eq!(form.saved_snapshot, host.saved);
        assert_eq!(form.save(&mut host), Ok(()));
        assert_eq!(host.writes, 1);
    }
    #[test]
    fn rejection_preserves_raw_input_and_never_promotes_it_to_saved() {
        let mut host = Host::new();
        let mut form = Form::open(&host);
        let before = host.saved.clone();
        assert_eq!(form.edit("500", ">>"), Ok(()));
        assert_eq!(form.save(&mut host), Err("invalid icon"));
        assert_eq!(form.raw_icon, ">>");
        assert_eq!(form.raw_increment, "500");
        assert_eq!(form.saved_snapshot, before);
        assert_eq!(host.saved, before);
        assert_eq!(host.writes, 0);
        form.cancel();
        assert_eq!(host.saved, before);
    }
    #[test]
    fn failed_write_can_retry_without_losing_draft() {
        let mut host = Host::new();
        let mut form = Form::open(&host);
        host.reject_write = true;
        assert_eq!(form.edit("500", ">"), Ok(()));
        assert_eq!(form.save(&mut host), Err("write failed"));
        assert_eq!(host.saved.increment, 10);
        assert_eq!(form.raw_increment, "500");
        host.reject_write = false;
        assert_eq!(form.save(&mut host), Ok(()));
        assert_eq!(host.writes, 1);
    }
    #[test]
    fn retry_checks_freshness_instead_of_rebasing_a_stale_proposal() {
        let mut host = Host::new();
        let mut form = Form::open(&host);
        assert_eq!(form.edit("50", ">"), Ok(()));
        host.revision += 1;
        assert_eq!(form.save(&mut host), Err("invalid or stale draft"));
        assert_eq!(host.writes, 0);
    }
    #[test]
    fn invalid_number_requires_correction_before_save() {
        let mut host = Host::new();
        let mut form = Form::open(&host);
        assert_eq!(form.edit("many", ">"), Err("invalid number"));
        assert_eq!(form.save(&mut host), Err("invalid or stale draft"));
        assert_eq!(host.writes, 0);
        assert_eq!(form.edit("25", ">"), Ok(()));
        assert_eq!(form.save(&mut host), Ok(()));
    }
}
