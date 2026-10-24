//! Headless compatibility probe using PhotoCraft's real Document and History types.
//! This is not a production frontend: production integration must use Session commands.
use edit_session::{EditSession, FinishError};
use photocraft_doc::{Color, ColorMode, DocId, Document, Layer, LayerId, SampleType, Size};
use photocraft_ops::{History, LayerTarget};
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};
use std::sync::Arc;

#[derive(PartialEq, Eq)]
struct Stamp(DocId, LayerId, u64);

struct Probe {
    doc: Arc<Document>,
    history: History,
    target: LayerId,
    other: LayerId,
    revision: u64,
    draft: Option<EditSession<f32, Stamp>>,
}

impl Probe {
    fn new() -> Result<Self, String> {
        let mut doc = Document::with_background(
            "Synthetic probe",
            Size::new(8, 8),
            ColorMode::Rgb,
            SampleType::U8,
            Color::WHITE,
        );
        let target = doc.layers.first().ok_or("missing fixture layer")?.id;
        let other = doc.insert_above(None, Layer::raster("Other target", doc.pixel_format()));
        Ok(Self {
            doc: Arc::new(doc),
            history: History::new(100),
            target,
            other,
            revision: 0,
            draft: None,
        })
    }

    fn stamp(&self) -> Stamp {
        Stamp(self.doc.id, self.target, self.revision)
    }

    fn value(&self) -> Result<f32, String> {
        self.doc
            .layer(self.target)
            .map(|layer| layer.opacity)
            .ok_or_else(|| "target is gone".into())
    }

    fn apply(&mut self, opacity: f32) -> Result<(), String> {
        let revision = self.revision.checked_add(1).ok_or("revision exhausted")?;
        let mut after = (*self.doc).clone();
        after
            .layer_mut(self.target)
            .ok_or("target is gone")?
            .opacity = opacity;
        self.history.set_current_layers(LayerTarget {
            active: Some(self.target),
            selected: vec![self.target],
        });
        self.history.record(
            "Probe opacity",
            self.doc.clone(),
            LayerTarget {
                active: Some(self.target),
                selected: vec![self.target],
            },
        );
        self.doc = Arc::new(after);
        self.revision = revision;
        Ok(())
    }

    fn run(&mut self, event: &Value) -> Result<(), String> {
        match event
            .get("op")
            .and_then(Value::as_str)
            .ok_or("missing op")?
        {
            "begin" => {
                if self.draft.is_some() {
                    return Err("already editing".into());
                }
                self.draft = Some(EditSession::new(self.value()?, self.stamp()));
            }
            "preview" => {
                let proposal = valid_opacity(event);
                self.draft.as_mut().ok_or("no draft")?.propose(proposal)?;
            }
            "commit" => {
                let live = self.value()?;
                let stamp = self.stamp();
                let check = self.draft.as_ref().ok_or("no draft")?.check(&live, &stamp);
                if let Err(error) = check {
                    if error != FinishError::InvalidProposal {
                        self.draft = None;
                    }
                    return Err(finish_error(error).into());
                }
                let draft = self.draft.take().ok_or("no draft")?;
                if let Some(change) = draft.finish(&live, &stamp).map_err(finish_error)? {
                    self.apply(change.after)?;
                }
            }
            "cancel" => {
                self.draft.take().ok_or("no draft")?.cancel();
            }
            "external" => {
                self.apply(valid_opacity(event)?)?;
            }
            "select-other" => {
                std::mem::swap(&mut self.target, &mut self.other);
            }
            "undo" | "redo" => {
                let revision = self.revision.checked_add(1).ok_or("revision exhausted")?;
                let next = if event["op"] == "undo" {
                    self.history.undo(self.doc.clone())
                } else {
                    self.history.redo(self.doc.clone())
                };
                if let Some((doc, layers)) = next {
                    self.doc = doc;
                    if let Some(target) = layers.active {
                        self.target = target;
                    }
                    self.revision = revision;
                }
            }
            "inspect" => {}
            _ => return Err("unknown op".into()),
        }
        Ok(())
    }

    fn state(&self, error: Option<String>) -> Value {
        json!({
            "committed": self.value().ok(),
            "preview": self.draft.as_ref().map(|d| *d.preview()),
            "editing": self.draft.is_some(),
            "valid": self.draft.as_ref().is_none_or(EditSession::is_valid),
            "undo_depth": self.history.past_len(),
            "can_redo": self.history.can_redo(),
            "revision": self.revision,
            "error": error
        })
    }
}

fn valid_opacity(event: &Value) -> Result<f32, String> {
    let value = event
        .get("value")
        .and_then(Value::as_f64)
        .ok_or("invalid opacity")?;
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err("invalid opacity".into());
    }
    Ok(value as f32)
}

fn finish_error(error: FinishError) -> &'static str {
    match error {
        FinishError::InvalidProposal => "invalid proposal",
        FinishError::ContextChanged => "context changed",
        FinishError::ValueChanged => "value changed",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut probe = Probe::new()?;
    let mut stdout = io::stdout().lock();
    for line in io::stdin().lock().lines() {
        let request: Value = serde_json::from_str(&line?)?;
        if request["protocol"] != 1
            || request.get("id").and_then(Value::as_u64).is_none()
            || !request["event"].is_object()
        {
            return Err("invalid protocol envelope".into());
        }
        let error = probe.run(&request["event"]).err();
        let response = json!({"protocol": 1, "id": request["id"], "state": probe.state(error)});
        writeln!(stdout, "{response}")?;
        stdout.flush()?;
    }
    Ok(())
}
