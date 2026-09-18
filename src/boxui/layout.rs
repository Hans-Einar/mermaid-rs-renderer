use super::frame::{Budget, validate_request};
use super::*;
use std::collections::BTreeMap;

const PAD: f64 = 8.0;
const GAP: f64 = 8.0;
#[derive(Debug, Clone, Copy)]
pub struct TextExtent {
    pub width: f64,
    pub height: f64,
    pub baseline: f64,
}
/// Borrowed synchronous host metrics. Coordinates and font size are SVG logical px.
/// A callback must return promptly; cooperative cancellation cannot interrupt it.
pub trait TextMetrics {
    fn measure(&self, text: &str) -> TextExtent;
    fn font_family(&self) -> &str;
    fn font_size(&self) -> f64;
}
/// Predictable example/test metrics, not a substitute for the host's real font metrics.
pub struct MonospaceMetrics;
impl TextMetrics for MonospaceMetrics {
    fn measure(&self, text: &str) -> TextExtent {
        TextExtent {
            width: text.chars().count() as f64 * 8.0,
            height: 18.0,
            baseline: 14.0,
        }
    }
    fn font_family(&self) -> &str {
        "monospace"
    }
    fn font_size(&self) -> f64 {
        14.0
    }
}
#[derive(Debug, Clone)]
pub struct LayoutItem {
    pub id: String,
    pub rect: Rect,
    pub lines: Vec<String>,
    pub line_height: f64,
    pub baseline: f64,
    /// The input edit surface or diagram content pane, excluding its label.
    pub content_rect: Option<Rect>,
}
#[derive(Debug, Clone)]
pub struct BoxUiLayout {
    pub items: Vec<LayoutItem>,
    pub diagnostics: Vec<Diagnostic>,
    /// Rectangle and baseline offset for the simulation marker, measured with the frame.
    pub simulation_footer: Option<(Rect, f64)>,
}

pub fn layout_boxui(request: &PrepareRequest, metrics: &dyn TextMetrics) -> Result<BoxUiLayout> {
    let budget = Budget::new(request.budget_ms, &|| false);
    validate_request(request)?;
    layout(request, metrics, &budget)
}
pub(crate) fn value_text(v: &ValueState) -> String {
    let text = match &v.value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    };
    match v.validity {
        Validity::Missing => "unbound / missing".into(),
        Validity::Stale => format!("{text} [stale]"),
        Validity::Current => text,
    }
}
pub(crate) fn label(n: &Node, s: &Snapshot) -> String {
    let base = n
        .text
        .as_ref()
        .or(n.label.as_ref())
        .cloned()
        .unwrap_or_default();
    if n.kind == Kind::Value {
        return format!(
            "{base}: {}",
            value_text(s.value(n.value_binding.as_ref().unwrap()))
        );
    }
    let mut text = base;
    if n.kind == Kind::Input {
        match s.value(n.value_binding.as_ref().unwrap()).validity {
            Validity::Missing => text.push_str(" [unbound / missing]"),
            Validity::Stale => text.push_str(" [stale]"),
            _ => (),
        }
    }
    if let Some(id) = &n.command_binding {
        let c = s.command(id);
        if !c.enabled {
            text.push_str(&format!(
                " [disabled: {}]",
                if c.reason.is_empty() {
                    "unavailable"
                } else {
                    &c.reason
                }
            ));
        }
    }
    text
}
struct Engine<'a, 'b> {
    r: &'a PrepareRequest,
    metrics: &'a dyn TextMetrics,
    budget: &'a Budget<'b>,
    intrinsic: BTreeMap<String, (f64, f64)>,
    needed: BTreeMap<String, f64>,
    items: Vec<LayoutItem>,
    diagnostics: Vec<Diagnostic>,
}
impl Engine<'_, '_> {
    fn extent(&self, text: &str) -> Result<TextExtent> {
        self.budget.check()?;
        let e = self.metrics.measure(text);
        if !e.width.is_finite()
            || !e.height.is_finite()
            || !e.baseline.is_finite()
            || e.width < 0.0
            || e.height <= 0.0
            || e.height > 8192.0
            || !(0.0..=e.height).contains(&e.baseline)
        {
            return Err(BoxUiError::new(
                "invalid-metrics",
                "Host returned nonfinite or invalid text metrics",
            ));
        }
        Ok(e)
    }
    fn intrinsic(&mut self, n: &Node) -> Result<(f64, f64)> {
        self.budget.check()?;
        let mut result = if n.kind.is_region() {
            let children = n
                .children()
                .iter()
                .map(|c| self.intrinsic(c).map(|v| (c, v)))
                .collect::<Result<Vec<_>>>()?;
            let row = n.kind == Kind::Row;
            let mut main = 0.0;
            let mut cross: f64 = 0.0;
            for (c, (w, h)) in children {
                main += self.main_min(c, if row { w } else { h })?;
                cross = cross.max(if row { h } else { w });
            }
            main += GAP * (n.children().len() - 1) as f64 + PAD * 2.0;
            cross += PAD * 2.0;
            if row { (main, cross) } else { (cross, main) }
        } else {
            let text = label(n, &self.r.snapshot);
            let h = self.extent(&text)?.height;
            let mut w: f64 = 0.0;
            for word in text.split_whitespace() {
                w = w.max(self.extent(word)?.width);
            }
            match n.kind {
                Kind::Text => (w + PAD * 2.0, h + PAD * 2.0),
                Kind::Input => (
                    96.0_f64.max(w + PAD * 2.0),
                    (h + PAD * 2.0 + 32.0).max(32.0),
                ),
                Kind::Diagram => (160.0_f64.max(w + PAD * 2.0), 160.0),
                _ => (96.0_f64.max(w + PAD * 2.0), (h + PAD * 2.0).max(32.0)),
            }
        };
        if let Some(h) = self.needed.get(&n.id) {
            result.1 = result.1.max(*h);
        }
        self.intrinsic.insert(n.id.clone(), result);
        Ok(result)
    }
    fn main_min(&self, n: &Node, intrinsic: f64) -> Result<f64> {
        let min = intrinsic.max(n.size.as_ref().and_then(|s| s.min).unwrap_or(0.0));
        if n.size
            .as_ref()
            .and_then(|s| s.max)
            .is_some_and(|max| max + 0.001 < min)
        {
            return Err(BoxUiError::new(
                "layout-no-space",
                "Maximum is smaller than content minimum",
            )
            .node(&n.id));
        }
        Ok(min)
    }
    fn wrap(&mut self, n: &Node, text: &str, width: f64) -> Result<(Vec<String>, TextExtent)> {
        let mut metric = self.extent("Mg")?;
        let mut lines = Vec::new();
        for paragraph in text.split('\n') {
            let mut line = String::new();
            for word in paragraph.split_whitespace() {
                self.budget.check()?;
                let trial = if line.is_empty() {
                    word.into()
                } else {
                    format!("{line} {word}")
                };
                if !line.is_empty() && self.extent(&trial)?.width > width {
                    lines.push(std::mem::take(&mut line));
                }
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(word);
                if self.extent(word)?.width > width + 0.001 {
                    let mut d =
                        BoxUiError::new("text-overflow", format!("Unbroken text clipped: {word}"))
                            .node(&n.id)
                            .diagnostic;
                    d.severity = "warning".into();
                    self.diagnostics.push(d);
                }
            }
            lines.push(line);
        }
        // Fallback fonts may have a different ascent/descent than Latin "Mg".
        let mut ascent = metric.baseline;
        let mut descent = metric.height - metric.baseline;
        for line in &lines {
            let extent = self.extent(line)?;
            ascent = ascent.max(extent.baseline);
            descent = descent.max(extent.height - extent.baseline);
        }
        if n.kind == Kind::Input {
            let extent = self.extent(&value_text(
                self.r.snapshot.value(n.value_binding.as_ref().unwrap()),
            ))?;
            ascent = ascent.max(extent.baseline);
            descent = descent.max(extent.height - extent.baseline);
        }
        metric.baseline = ascent;
        metric.height = ascent + descent;
        Ok((lines, metric))
    }
    fn allocate(&mut self, n: &Node, rect: Rect, final_pass: bool) -> Result<()> {
        self.budget.check()?;
        let (iw, ih) = self.intrinsic[&n.id];
        if rect.width + 0.001 < iw || (final_pass && rect.height + 0.001 < ih) {
            return Err(BoxUiError::new(
                "layout-no-space",
                format!(
                    "{} needs at least {:.1} × {:.1} px; has {:.1} × {:.1}",
                    n.id, iw, ih, rect.width, rect.height
                ),
            )
            .node(&n.id));
        }
        let mut item = LayoutItem {
            id: n.id.clone(),
            rect,
            lines: vec![],
            line_height: 0.0,
            baseline: 0.0,
            content_rect: None,
        };
        if !n.kind.is_region() {
            let text = label(n, &self.r.snapshot);
            let (lines, metric) = self.wrap(n, &text, (rect.width - 2.0 * PAD).max(0.0))?;
            let label_height = lines.len() as f64 * metric.height + 2.0 * PAD;
            let edit_height = (metric.height + 16.0).max(32.0);
            let required = label_height
                + if n.kind == Kind::Input {
                    edit_height
                } else if n.kind == Kind::Diagram {
                    32.0
                } else {
                    0.0
                };
            self.needed.insert(n.id.clone(), required);
            if final_pass && required > rect.height + 0.001 {
                return Err(BoxUiError::new(
                    "layout-no-space",
                    "Wrapped content exceeds allocated height",
                )
                .node(&n.id));
            }
            item.lines = lines;
            item.line_height = metric.height;
            item.baseline = metric.baseline;
            if matches!(n.kind, Kind::Input | Kind::Diagram) {
                item.content_rect = Some(Rect {
                    x: rect.x + PAD,
                    y: rect.y + label_height,
                    width: (rect.width - 2.0 * PAD).max(0.0),
                    height: if n.kind == Kind::Input {
                        edit_height
                    } else {
                        (rect.height - label_height - PAD).max(0.0)
                    },
                });
            }
            self.items.push(item);
            return Ok(());
        }
        self.items.push(item);
        let row = n.kind == Kind::Row;
        let available = (if row { rect.width } else { rect.height })
            - PAD * 2.0
            - GAP * (n.children().len() - 1) as f64;
        let mut sizes = Vec::new();
        for c in n.children() {
            let (w, h) = self.intrinsic[&c.id];
            sizes.push(self.main_min(c, if row { w } else { h })?);
        }
        let total = sizes.iter().sum::<f64>();
        if available + 0.001 < total {
            return Err(
                BoxUiError::new("layout-no-space", "Region cannot fit child minima").node(&n.id),
            );
        }
        let mut remaining = (available - total).max(0.0);
        let mut active: Vec<usize> = n
            .children()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                (c.grow() > 0.0
                    && c.size
                        .as_ref()
                        .and_then(|s| s.max)
                        .is_none_or(|max| max > sizes[i] + 0.001))
                .then_some(i)
            })
            .collect();
        // Each clamping round removes at least one child; no unbounded reflow.
        while remaining > 0.001 && !active.is_empty() {
            self.budget.check()?;
            let weight: f64 = active.iter().map(|&i| n.children()[i].grow()).sum();
            let mut used = 0.0;
            for &i in &active {
                let c = &n.children()[i];
                let max = c.size.as_ref().and_then(|s| s.max).unwrap_or(f64::INFINITY);
                let extra = (remaining * c.grow() / weight).min((max - sizes[i]).max(0.0));
                sizes[i] += extra;
                used += extra;
            }
            remaining = (remaining - used).max(0.0);
            active.retain(|&i| {
                n.children()[i]
                    .size
                    .as_ref()
                    .and_then(|s| s.max)
                    .is_none_or(|max| max > sizes[i] + 0.001)
            });
            if used < 0.001 {
                break;
            }
        }
        let mut cursor = if row { rect.x + PAD } else { rect.y + PAD };
        for (c, size) in n.children().iter().zip(sizes) {
            let child = if row {
                Rect {
                    x: cursor,
                    y: rect.y + PAD,
                    width: size,
                    height: rect.height - 2.0 * PAD,
                }
            } else {
                Rect {
                    x: rect.x + PAD,
                    y: cursor,
                    width: rect.width - 2.0 * PAD,
                    height: size,
                }
            };
            self.allocate(c, child, final_pass)?;
            cursor += size + GAP;
        }
        Ok(())
    }
}
pub(crate) fn layout(
    r: &PrepareRequest,
    m: &dyn TextMetrics,
    budget: &Budget<'_>,
) -> Result<BoxUiLayout> {
    if !m.font_size().is_finite()
        || !(1.0..=512.0).contains(&m.font_size())
        || m.font_family().len() > 256
        || m.font_family().is_empty()
    {
        return Err(BoxUiError::new(
            "invalid-metrics",
            "Invalid host font description",
        ));
    }
    let mut e = Engine {
        r,
        metrics: m,
        budget,
        intrinsic: BTreeMap::new(),
        needed: BTreeMap::new(),
        items: vec![],
        diagnostics: vec![],
    };
    let simulation_footer = if r.snapshot.simulated() {
        let extent = e.extent("simulated snapshot")?;
        let height = (extent.height + 8.0).max(24.0);
        if extent.width + 16.0 > r.viewport.width || height >= r.viewport.height {
            return Err(BoxUiError::new(
                "layout-no-space",
                "Simulation marker cannot fit the viewport",
            ));
        }
        Some((
            Rect {
                x: 0.0,
                y: r.viewport.height - height,
                width: r.viewport.width,
                height,
            },
            4.0 + extent.baseline,
        ))
    } else {
        None
    };
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: r.viewport.width,
        height: simulation_footer.map_or(r.viewport.height, |(rect, _)| rect.y),
    };
    e.intrinsic(&r.model.root)?;
    e.allocate(&r.model.root, rect, false)?;
    e.items.clear();
    e.diagnostics.clear();
    e.intrinsic(&r.model.root)?;
    e.allocate(&r.model.root, rect, true)?;
    Ok(BoxUiLayout {
        items: e.items,
        diagnostics: e.diagnostics,
        simulation_footer,
    })
}
