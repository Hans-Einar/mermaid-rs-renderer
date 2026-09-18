use super::frame::*;
use super::{model::*, svg, validate};
use anyhow::{Result, ensure};
use std::{collections::BTreeMap, time::Instant};
struct Layout<'a> {
    start: Instant,
    budget: u64,
    cancel: &'a dyn Fn() -> bool,
    measure: &'a dyn Fn(&str) -> (f64, f64),
    placed: Vec<(&'a Node, Rect)>,
}
impl Layout<'_> {
    fn checkpoint(&self) -> Result<()> {
        ensure!(!(self.cancel)(), "BoxUI cancelled");
        ensure!(
            self.start.elapsed().as_millis() <= self.budget as u128,
            "BoxUI layout time budget exceeded"
        );
        Ok(())
    }
    fn text_height(&self, text: &str, width: f64) -> Result<f64> {
        let mut lines = 1.;
        let mut used = 0.;
        for word in text.split_whitespace() {
            let (w, h) = (self.measure)(word);
            ensure!(
                w.is_finite() && h.is_finite() && w >= 0. && h > 0.,
                "Invalid text metrics"
            );
            if used > 0. && used + w + 5. > width {
                lines += 1.;
                used = 0.;
            }
            used += w + 5.;
        }
        Ok(lines * 20.)
    }
    fn min(&self, n: &Node, width: f64) -> Result<(f64, f64)> {
        self.checkpoint()?;
        let (w, h) = match n.kind {
            Kind::Row => {
                let mut w = 0.;
                let mut h: f64 = 0.;
                for c in &n.children {
                    let (cw, ch) = self.min(c, (width - 16.) / n.children.len() as f64)?;
                    w += cw;
                    h = h.max(ch);
                }
                (w + 8. * (n.children.len() + 1) as f64, h + 16.)
            }
            Kind::Column => {
                let mut w: f64 = 0.;
                let mut h = 0.;
                for c in &n.children {
                    let (cw, ch) = self.min(c, width - 16.)?;
                    w = w.max(cw);
                    h += ch;
                }
                (w + 16., h + 8. * (n.children.len() + 1) as f64)
            }
            Kind::Diagram => (160., 160.),
            Kind::Input => (120., 60.),
            Kind::Button => {
                let (w, _) = (self.measure)(n.label.as_deref().unwrap_or(""));
                (w.max(80.) + 16., 36.)
            }
            Kind::Value => (120., 44.),
            Kind::Text => (
                40.,
                self.text_height(n.text.as_deref().unwrap_or(""), width.max(1.))?
                    .max(24.),
            ),
        };
        Ok((w, h))
    }
}
fn place<'a>(l: &mut Layout<'a>, n: &'a Node, r: Rect) -> Result<()> {
    l.checkpoint()?;
    if !matches!(n.kind, Kind::Row | Kind::Column) {
        l.placed.push((n, r));
        return Ok(());
    }
    let row = n.kind == Kind::Row;
    let available = if row { r.width } else { r.height } - 8. * (n.children.len() + 1) as f64;
    let cross = if row { r.height } else { r.width } - 16.;
    ensure!(cross > 0., "layout-no-space: {}", n.id);
    let mut sizes = Vec::new();
    let mut grows = Vec::new();
    let mut maxes = Vec::new();
    for c in &n.children {
        let (w, h) = l.min(
            c,
            if row {
                r.width / n.children.len() as f64
            } else {
                cross
            },
        )?;
        let intrinsic = if row { w } else { h };
        let min = intrinsic.max(c.size.min.unwrap_or(0.));
        let max = c.size.max.unwrap_or(8192.);
        ensure!(
            max >= min,
            "layout-no-space: {} max below content minimum",
            c.id
        );
        sizes.push(min);
        maxes.push(max);
        grows.push(c.size.grow.unwrap_or(
            if matches!(c.kind, Kind::Diagram | Kind::Row | Kind::Column) {
                1.
            } else {
                0.
            },
        ));
    }
    let mut extra = available - sizes.iter().sum::<f64>();
    ensure!(extra >= -0.01, "layout-no-space: {}", n.id);
    for _ in 0..=sizes.len() {
        let total: f64 = grows.iter().sum();
        if total <= 0. || extra <= 0.01 {
            break;
        }
        let start = extra;
        for i in 0..sizes.len() {
            let add = (start * grows[i] / total).min(maxes[i] - sizes[i]);
            sizes[i] += add;
            extra -= add;
            if sizes[i] >= maxes[i] - 0.01 {
                grows[i] = 0.;
            }
        }
    }
    let mut cursor = if row { r.x } else { r.y } + 8.;
    for (c, size) in n.children.iter().zip(sizes) {
        let child = if row {
            Rect {
                x: cursor,
                y: r.y + 8.,
                width: size,
                height: cross,
            }
        } else {
            Rect {
                x: r.x + 8.,
                y: cursor,
                width: cross,
                height: size,
            }
        };
        place(l, c, child)?;
        cursor += size + 8.;
    }
    Ok(())
}
/// Stateless preparation; host owns values, enabled-state authority and text editing.
/// Child scenes must have been parsed/prepared by the caller before this stage.
pub fn prepare(
    doc: &Document,
    snapshot: &Snapshot,
    children: &BTreeMap<String, ChildScene>,
    width: f64,
    height: f64,
    budget_ms: u64,
    measure: &dyn Fn(&str) -> (f64, f64),
    cancel: &dyn Fn() -> bool,
) -> Result<Frame> {
    validate(doc)?;
    ensure!(
        width.is_finite()
            && height.is_finite()
            && (1.0..=8192.0).contains(&width)
            && (1.0..=8192.0).contains(&height),
        "Invalid viewport"
    );
    ensure!((1..=10000).contains(&budget_ms), "Invalid budget");
    ensure!(children.len() <= 8, "Child budget");
    for (id, v) in &snapshot.values {
        let b = doc
            .bindings
            .iter()
            .find(|b| b.id == *id && b.role == "value")
            .ok_or_else(|| anyhow::anyhow!("Unknown value binding"))?;
        ensure!(
            ["missing", "current", "stale"].contains(&v.validity.as_str()),
            "Invalid currentness"
        );
        ensure!(
            if v.validity == "missing" {
                v.value.is_null()
            } else {
                match b.r#type.as_str() {
                    "string" => v.value.is_string(),
                    "number" => v.value.is_number(),
                    "boolean" => v.value.is_boolean(),
                    _ => false,
                }
            },
            "Snapshot type mismatch"
        );
    }
    let mut l = Layout {
        start: Instant::now(),
        budget: budget_ms,
        cancel,
        measure,
        placed: Vec::new(),
    };
    place(
        &mut l,
        &doc.root,
        Rect {
            x: 0.,
            y: 24.,
            width,
            height: height - 24.,
        },
    )?;
    let mut frame = Frame {
        width,
        height,
        static_svg: String::new(),
        preview_svg: String::new(),
        controls: Vec::new(),
        diagnostics: Vec::new(),
    };
    let mut content = String::new();
    let mut preview = String::new();
    for (n, r) in &l.placed {
        l.checkpoint()?;
        let enabled = n
            .command_binding
            .as_ref()
            .and_then(|id| snapshot.enabled.get(id))
            .copied()
            .unwrap_or(false);
        let value = n
            .value_binding
            .as_ref()
            .and_then(|id| snapshot.values.get(id));
        let text = value
            .map(|v| {
                if v.validity == "missing" {
                    "missing".into()
                } else {
                    let s = if let Some(s) = v.value.as_str() {
                        s.to_string()
                    } else {
                        v.value.to_string()
                    };
                    if v.validity == "stale" {
                        format!("{s} (stale)")
                    } else {
                        s
                    }
                }
            })
            .unwrap_or_else(|| "unbound".into());
        let (paint, edit) = svg::widget(
            n,
            *r,
            &text,
            enabled,
            children,
            &mut frame.diagnostics,
            measure,
        )?;
        content.push_str(&paint);
        preview.push_str(&edit);
        if matches!(n.kind, Kind::Input | Kind::Button) {
            let mut rect = *r;
            if n.kind == Kind::Input {
                rect.y += 24.;
                rect.height = 32.;
            }
            frame.controls.push(Control {
                id: n.id.clone(),
                kind: n.kind.clone(),
                version: 1,
                rect,
                clip: Rect {
                    x: 0.,
                    y: 0.,
                    width,
                    height,
                },
                enabled,
                role: if n.kind == Kind::Input {
                    "textbox"
                } else {
                    "button"
                }
                .into(),
                accessible_name: n.label.clone().unwrap_or_default(),
                command_binding: n.command_binding.clone().unwrap(),
                value_binding: n.value_binding.clone(),
                value_type: if n.kind == Kind::Input {
                    "string"
                } else {
                    "none"
                }
                .into(),
            });
        }
    }
    frame.static_svg = svg::document(width, height, &content, snapshot.simulated);
    frame.preview_svg = svg::document(width, height, &preview, snapshot.simulated);
    ensure!(
        frame.static_svg.len() + frame.preview_svg.len() < 8 * 1024 * 1024,
        "Frame exceeds 8 MiB"
    );
    Ok(frame)
}
