use super::embedded::xml;
use super::frame::{Budget, validate_request};
use super::*;
use std::collections::BTreeMap;
use std::fmt::Write;

/// Prepare both SVG variants and controls atomically. No source parsing, I/O,
/// command execution, native widgets or globally retained frame state occurs here.
pub fn prepare_boxui(r: &PrepareRequest, metrics: &dyn TextMetrics) -> Result<BoxUiFrame> {
    prepare_boxui_cancellable(r, metrics, &|| false)
}
pub fn prepare_boxui_cancellable(
    r: &PrepareRequest,
    metrics: &dyn TextMetrics,
    cancelled: &dyn Fn() -> bool,
) -> Result<BoxUiFrame> {
    let budget = Budget::new(r.budget_ms, cancelled);
    budget.check()?;
    validate_request(r)?;
    let layout = super::layout::layout(r, metrics, &budget)?;
    let mut diagnostics = layout.diagnostics.clone();
    let mut children = BTreeMap::new();
    fn find<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
        if n.id == id {
            return Some(n);
        }
        n.children().iter().find_map(|c| find(c, id))
    }
    for (index, c) in r.children.iter().enumerate() {
        budget.check()?;
        let node = find(&r.model.root, c.reference()).unwrap();
        let profile = CHILD_PROFILES
            .iter()
            .find(|(family, _)| Some(*family) == node.family.as_deref())
            .unwrap()
            .1;
        let result = if !r.child_profiles.iter().any(|p| p == profile) {
            Err(BoxUiError::new(
                "child-profile",
                format!("Host does not advertise {profile}"),
            ))
        } else {
            match c {
                PreparedChild::Svg {
                    svg, width, height, ..
                } => super::embedded::sanitize(svg, &format!("bx-child-{index}"), &budget)
                    .map(|svg| (*width, *height, svg)),
                PreparedChild::Error { error, .. } => Err(BoxUiError::new("child-error", error)),
            }
        };
        if let Err(e) = &result {
            if matches!(e.diagnostic.code.as_str(), "cancelled" | "budget-exceeded") {
                return Err(e.clone());
            }
            diagnostics.push(e.clone().node(c.reference()).diagnostic);
        }
        children.insert(c.reference(), result);
    }
    let items: BTreeMap<_, _> = layout.items.iter().map(|i| (i.id.as_str(), i)).collect();
    let mut controls = Vec::new();
    fn controls_for(
        n: &Node,
        r: &PrepareRequest,
        items: &BTreeMap<&str, &LayoutItem>,
        out: &mut Vec<Control>,
    ) {
        if let Some(binding) = &n.command_binding {
            let i = items[n.id.as_str()];
            let rect = i.content_rect.unwrap_or(i.rect);
            if rect.width > 0.0 && rect.height > 0.0 {
                out.push(Control {
                    id: n.id.clone(),
                    kind: n.kind,
                    version: n.version,
                    rect,
                    clip: rect,
                    enabled: r.snapshot.command(binding).enabled,
                    role: if n.kind == Kind::Input {
                        "textbox"
                    } else {
                        "button"
                    }
                    .into(),
                    accessible_name: n.label.clone().unwrap(),
                    command_binding: binding.clone(),
                    value_binding: n.value_binding.clone(),
                    value_type: if n.kind == Kind::Input {
                        DataType::String
                    } else {
                        DataType::None
                    },
                });
            }
        }
        for c in n.children() {
            controls_for(c, r, items, out);
        }
    }
    controls_for(&r.model.root, r, &items, &mut controls);
    let mut paint = Painter {
        r,
        metrics,
        items,
        children,
        budget: &budget,
    };
    let static_svg = paint.render(false)?;
    let preview_svg = paint.render(true)?;
    let frame = BoxUiFrame {
        contract: CONTRACT.into(),
        key: r.key.clone(),
        width: r.viewport.width,
        height: r.viewport.height,
        static_svg,
        preview_svg,
        controls,
        diagnostics,
    };
    if serde_json::to_vec(&frame)
        .map_err(|e| BoxUiError::new("frame-encoding", e.to_string()))?
        .len()
        > MAX_FRAME_BYTES
    {
        return Err(BoxUiError::new(
            "frame-budget",
            "Complete frame exceeds 8 MiB",
        ));
    }
    budget.check()?;
    Ok(frame)
}
type ChildScene = std::result::Result<(f64, f64, String), BoxUiError>;
struct Painter<'a, 'b> {
    r: &'a PrepareRequest,
    metrics: &'a dyn TextMetrics,
    items: BTreeMap<&'a str, &'a LayoutItem>,
    children: BTreeMap<&'a str, ChildScene>,
    budget: &'a Budget<'b>,
}
impl Painter<'_, '_> {
    fn rect(&self, s: &mut String, r: Rect, fill: &str, border: &str) {
        write!(
            s,
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\"/>",
            r.x, r.y, r.width, r.height, fill, border
        )
        .unwrap();
    }
    fn render(&mut self, preview: bool) -> Result<String> {
        let mut s = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" font-family=\"{}\" font-size=\"{}\"><title>BoxUI {}</title>",
            self.r.viewport.width,
            self.r.viewport.height,
            self.r.viewport.width,
            self.r.viewport.height,
            xml(self.metrics.font_family()),
            self.metrics.font_size(),
            xml(&self.r.model.document_id)
        );
        self.rect(
            &mut s,
            Rect {
                x: 0.0,
                y: 0.0,
                width: self.r.viewport.width,
                height: self.r.viewport.height,
            },
            &self.r.palette.background,
            "none",
        );
        self.node(&self.r.model.root, &mut s, preview)?;
        if self.r.snapshot.simulated() {
            write!(
                s,
                "<text x=\"8\" y=\"{}\" fill=\"{}\">simulated snapshot</text>",
                self.r.viewport.height - 8.0,
                self.r.palette.muted
            )
            .unwrap();
        }
        s.push_str("</svg>");
        Ok(s)
    }
    fn node(&self, n: &Node, s: &mut String, preview: bool) -> Result<()> {
        self.budget.check()?;
        let i = self.items[n.id.as_str()];
        let r = i.rect;
        let clip = format!("bx-node-{}", n.id);
        write!(s,"<defs><clipPath id=\"{clip}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs><g clip-path=\"url(#{clip})\"><title>{}</title>",r.x,r.y,r.width,r.height,xml(&super::layout::label(n,&self.r.snapshot))).unwrap();
        if n.kind == Kind::Button {
            self.rect(s, r, &self.r.palette.surface, &self.r.palette.border);
        }
        let disabled = n
            .command_binding
            .as_ref()
            .is_some_and(|id| !self.r.snapshot.command(id).enabled);
        let color = if disabled {
            &self.r.palette.muted
        } else {
            &self.r.palette.foreground
        };
        for (index, line) in i.lines.iter().enumerate() {
            write!(
                s,
                "<text x=\"{}\" y=\"{}\" fill=\"{color}\">{}</text>",
                r.x + 8.0,
                r.y + 8.0 + i.baseline + index as f64 * i.line_height,
                xml(line)
            )
            .unwrap();
        }
        if n.kind == Kind::Input {
            let edit = i.content_rect.unwrap();
            // Preview paints only the outer outline; the host owns its native edit interior.
            self.rect(
                s,
                edit,
                if preview {
                    "none"
                } else {
                    &self.r.palette.surface
                },
                &self.r.palette.border,
            );
            if !preview {
                let clip = format!("bx-edit-{}", n.id);
                write!(s,"<defs><clipPath id=\"{clip}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs><text clip-path=\"url(#{clip})\" x=\"{}\" y=\"{}\" fill=\"{color}\">{}</text>",
                    edit.x+4.0,edit.y,edit.width-8.0,edit.height,edit.x+4.0,edit.y+8.0+i.baseline,xml(&super::layout::value_text(self.r.snapshot.value(n.value_binding.as_ref().unwrap())))).unwrap();
            }
        }
        if n.kind == Kind::Diagram {
            let pane = i.content_rect.unwrap();
            let child = &self.children[n.id.as_str()];
            let clip = format!("bx-pane-{}", n.id);
            write!(s,"<defs><clipPath id=\"{clip}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs><g clip-path=\"url(#{clip})\">",pane.x,pane.y,pane.width,pane.height).unwrap();
            match child {
                Ok((w, h, svg)) => {
                    let scale = (pane.width / w).min(pane.height / h);
                    let x = pane.x + (pane.width - w * scale) / 2.0;
                    let y = pane.y + (pane.height - h * scale) / 2.0;
                    // Nested viewport supplies declared logical size even if child SVG uses percentages.
                    write!(s,"<svg x=\"{x}\" y=\"{y}\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {w} {h}\">{svg}</svg>",w*scale,h*scale).unwrap();
                }
                Err(e) => {
                    write!(s,"<title>{}</title><text x=\"{}\" y=\"{}\" fill=\"{}\">Diagram unavailable: {}</text>",xml(&e.diagnostic.message),pane.x+4.0,pane.y+20.0,self.r.palette.error,xml(&e.diagnostic.message)).unwrap();
                }
            }
            s.push_str("</g>");
        }
        for c in n.children() {
            self.node(c, s, preview)?;
        }
        s.push_str("</g>");
        if s.len() > MAX_FRAME_BYTES {
            return Err(BoxUiError::new("frame-budget", "SVG exceeds frame budget"));
        }
        Ok(())
    }
}
