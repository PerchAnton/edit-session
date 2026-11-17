//! Host model of a nested confirmation. Not a COSMIC GUI test or upstream patch.
use edit_session::EditSession;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Shortcut {
    name: String,
    command: String,
    binding: String,
}

// A correct small local helper is a valid alternative to the library.
enum Draft {
    Library(EditSession<Shortcut, u64>),
    Local {
        original: Shortcut,
        proposal: Shortcut,
        revision: u64,
    },
}
impl Draft {
    fn open(library: bool, host: &Host) -> Self {
        if library {
            Self::Library(EditSession::new(host.value.clone(), host.revision))
        } else {
            Self::Local {
                original: host.value.clone(),
                proposal: host.value.clone(),
                revision: host.revision,
            }
        }
    }
    fn propose(&mut self, value: Shortcut) -> Result<(), &'static str> {
        match self {
            Self::Library(draft) => draft.propose(Ok(value)),
            Self::Local { proposal, .. } => {
                *proposal = value;
                Ok(())
            }
        }
    }
    fn check(&self, host: &Host) -> Result<&Shortcut, &'static str> {
        match self {
            Self::Library(draft) => {
                draft
                    .check(&host.value, &host.revision)
                    .map_err(|_| "stale draft")?;
                Ok(draft.preview())
            }
            Self::Local {
                original,
                proposal,
                revision,
            } => {
                if *revision != host.revision || *original != host.value {
                    Err("stale draft")
                } else {
                    Ok(proposal)
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Host {
    value: Shortcut,
    revision: u64,
    writes: usize,
}
struct Form {
    fields: Shortcut,
    draft: Option<Draft>,
    replace_pending: bool,
}
impl Form {
    fn open(library: bool, host: &Host) -> Self {
        Self {
            fields: host.value.clone(),
            draft: Some(Draft::open(library, host)),
            replace_pending: false,
        }
    }
    fn edit(
        &mut self,
        name: &str,
        command: &str,
        binding: &str,
        occupied: bool,
    ) -> Result<(), &'static str> {
        let draft = self.draft.as_mut().ok_or("closed")?;
        self.fields = Shortcut {
            name: name.into(),
            command: command.into(),
            binding: binding.into(),
        };
        draft.propose(self.fields.clone())?;
        self.replace_pending = occupied;
        Ok(())
    }
    fn cancel_replace(&mut self) {
        // Only the child confirmation ends. Keep the parent draft and raw fields.
        self.replace_pending = false;
        self.fields.binding.clear();
        if let Some(draft) = self.draft.as_mut() {
            let _ = draft.propose(self.fields.clone());
        }
    }
    fn cancel_form(&mut self) {
        self.draft = None;
        self.replace_pending = false;
    }
    fn accept_replace(&mut self) {
        self.replace_pending = false;
    }
    fn confirm(&mut self, host: &mut Host) -> Result<(), &'static str> {
        if self.replace_pending {
            return Err("replacement needs confirmation");
        }
        let value = self.draft.as_ref().ok_or("closed")?.check(host)?;
        if value.name.trim().is_empty()
            || value.command.trim().is_empty()
            || value.binding.is_empty()
        {
            return Err("complete the fields");
        }
        if *value != host.value {
            let revision = host.revision.checked_add(1).ok_or("revision exhausted")?;
            // Serialized host command: no callback/await between check and write.
            host.value = value.clone();
            host.revision = revision;
            host.writes += 1;
        }
        self.cancel_form();
        Ok(())
    }
}

fn main() -> Result<(), &'static str> {
    let mut host = Host::default();
    let mut form = Form::open(true, &host);
    form.edit("Terminal", "terminal", "Super+H", true)?;
    form.cancel_replace();
    assert_eq!(form.fields.name, "Terminal");
    assert_eq!(form.fields.command, "terminal");
    form.edit("Terminal", "terminal", "Super+T", false)?;
    form.confirm(&mut host)?;
    assert_eq!(host.writes, 1);
    let mut replacement = Form::open(true, &host);
    replacement.edit("Terminal", "terminal", "Super+H", true)?;
    replacement.accept_replace();
    replacement.confirm(&mut host)?;
    assert_eq!(host.writes, 2);
    println!("Child Cancel retained the parent fields; choosing another binding committed once.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn library_and_local_helper_preserve_parent_fields_on_child_cancel() {
        for library in [false, true] {
            let mut host = Host::default();
            let mut form = Form::open(library, &host);
            assert_eq!(form.edit("Terminal", "terminal", "Super+H", true), Ok(()));
            assert_eq!(
                form.confirm(&mut host),
                Err("replacement needs confirmation")
            );
            form.cancel_replace();
            assert_eq!(
                (&*form.fields.name, &*form.fields.command),
                ("Terminal", "terminal")
            );
            assert_eq!(host.value, Shortcut::default());
            assert_eq!(host.writes, 0);
            assert_eq!(form.confirm(&mut host), Err("complete the fields"));
            assert_eq!(form.edit("Terminal", "terminal", "Super+T", false), Ok(()));
            assert_eq!(form.confirm(&mut host), Ok(()));
            assert_eq!(host.value.binding, "Super+T");
            assert_eq!(host.writes, 1);
            assert_eq!(form.confirm(&mut host), Err("closed"));
        }
    }
    #[test]
    fn cancel_entire_form_never_writes() {
        for library in [false, true] {
            let mut host = Host::default();
            let mut form = Form::open(library, &host);
            assert_eq!(form.edit("Terminal", "terminal", "Super+H", true), Ok(()));
            form.cancel_form();
            assert_eq!(form.confirm(&mut host), Err("closed"));
            assert_eq!(host.value, Shortcut::default());
            assert_eq!(host.writes, 0);
        }
    }
    #[test]
    fn confirmation_rechecks_revision_and_current_target_value() {
        for library in [false, true] {
            for change_revision in [false, true] {
                let mut host = Host::default();
                let mut form = Form::open(library, &host);
                assert_eq!(form.edit("Terminal", "terminal", "Super+H", true), Ok(()));
                if change_revision {
                    host.revision += 1;
                } else {
                    host.value.command = "other".into();
                }
                let before = host.value.clone();
                form.accept_replace();
                assert_eq!(form.confirm(&mut host), Err("stale draft"));
                assert_eq!(host.value, before);
                assert_eq!(host.writes, 0);
                assert!(form.draft.is_some());
            }
        }
    }
    #[test]
    fn accepted_replacement_is_one_host_commit() {
        for library in [false, true] {
            let mut host = Host::default();
            let mut form = Form::open(library, &host);
            assert_eq!(form.edit("Terminal", "terminal", "Super+H", true), Ok(()));
            form.accept_replace();
            assert_eq!(form.confirm(&mut host), Ok(()));
            assert_eq!(host.writes, 1);
            assert_eq!(host.value.binding, "Super+H");
        }
    }
    #[test]
    fn returning_to_an_equal_configuration_does_not_make_an_old_confirmation_fresh() {
        for library in [false, true] {
            let mut host = Host::default();
            let mut form = Form::open(library, &host);
            assert_eq!(form.edit("Terminal", "terminal", "Super+H", true), Ok(()));
            let original = host.value.clone();
            host.value.command = "external".into();
            host.revision += 1;
            host.value = original;
            host.revision += 1;
            form.accept_replace();
            assert_eq!(form.confirm(&mut host), Err("stale draft"));
            assert_eq!(host.writes, 0);
            assert_eq!(form.fields.command, "terminal");
        }
    }
}
