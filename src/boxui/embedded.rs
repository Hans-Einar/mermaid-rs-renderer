//! A deliberately narrow inert SVG composition profile. Unknown constructs fail
//! locally; do not pass uninspected child XML or CSS into the parent SVG.
use super::frame::Budget;
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

pub(crate) fn xml(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            '\t' | '\n' | '\r' => out.push(c),
            c if c < '\u{20}' || c == '\u{fffe}' || c == '\u{ffff}' => out.push('\u{fffd}'),
            c => out.push(c),
        }
    }
    out
}
const SVG_NS: &str = "http://www.w3.org/2000/svg";
const ELEMENTS: &[&str] = &[
    "svg",
    "g",
    "defs",
    "path",
    "rect",
    "text",
    "tspan",
    "line",
    "polyline",
    "polygon",
    "circle",
    "ellipse",
    "marker",
    "clipPath",
    "linearGradient",
    "radialGradient",
    "stop",
    "title",
    "desc",
];
const ATTRS: &[&str] = &[
    "id",
    "class",
    "x",
    "y",
    "x1",
    "x2",
    "y1",
    "y2",
    "dx",
    "dy",
    "width",
    "height",
    "viewBox",
    "preserveAspectRatio",
    "d",
    "points",
    "cx",
    "cy",
    "r",
    "rx",
    "ry",
    "transform",
    "fill",
    "fill-opacity",
    "fill-rule",
    "stroke",
    "stroke-width",
    "stroke-opacity",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-miterlimit",
    "opacity",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "text-anchor",
    "dominant-baseline",
    "alignment-baseline",
    "textLength",
    "lengthAdjust",
    "letter-spacing",
    "word-spacing",
    "marker-start",
    "marker-mid",
    "marker-end",
    "markerWidth",
    "markerHeight",
    "markerUnits",
    "refX",
    "refY",
    "orient",
    "clip-path",
    "clipPathUnits",
    "gradientUnits",
    "gradientTransform",
    "offset",
    "stop-color",
    "stop-opacity",
    "fx",
    "fy",
    "spreadMethod",
    "vector-effect",
    "paint-order",
    "visibility",
    "display",
    "shape-rendering",
    "text-rendering",
    "overflow",
    "style",
    "role",
    "aria-label",
    "aria-hidden",
    "data-id",
];
const CSS: &[&str] = &[
    "fill",
    "stroke",
    "stroke-width",
    "stroke-dasharray",
    "stroke-linecap",
    "stroke-linejoin",
    "fill-opacity",
    "stroke-opacity",
    "opacity",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "text-anchor",
    "dominant-baseline",
    "alignment-baseline",
    "color",
    "background",
    "background-color",
    "max-width",
    "height",
    "width",
    "aspect-ratio",
    "mix-blend-mode",
    "white-space",
    "overflow",
    "display",
    "visibility",
];

fn bad(s: &str) -> BoxUiError {
    BoxUiError::new("child-svg-unsupported", s)
}
fn value(s: &str, ids: &BTreeMap<String, String>) -> Result<String> {
    // Reject CSS escapes/comments/control characters rather than trying to decode another language.
    if s.contains(['\\', '<', '>', '@'])
        || s.contains("/*")
        || s.chars()
            .any(|c| c < ' ' && !matches!(c, '\n' | '\r' | '\t'))
    {
        return Err(bad("Unsupported child attribute syntax"));
    }
    let lower = s.to_ascii_lowercase();
    if lower.contains("url") {
        if !s.starts_with("url(#") || !s.ends_with(')') {
            return Err(bad("Only exact local url(#id) references are allowed"));
        }
        let id = &s[5..s.len() - 1];
        let target = ids
            .get(id)
            .ok_or_else(|| bad("Unresolved child SVG reference"))?;
        return Ok(format!("url(#{target})"));
    }
    if lower.contains("://")
        || lower.contains("data:")
        || lower.contains("javascript:")
        || lower.contains("expression(")
    {
        return Err(bad("External or executable child attribute"));
    }
    Ok(s.into())
}
pub(crate) fn sanitize(svg: &str, prefix: &str, budget: &Budget<'_>) -> Result<String> {
    budget.check()?;
    if svg.len() > MAX_FRAME_BYTES {
        return Err(bad("Child SVG exceeds budget"));
    }
    let doc = roxmltree::Document::parse_with_options(
        svg,
        roxmltree::ParsingOptions {
            allow_dtd: false,
            nodes_limit: 100_000,
            ..Default::default()
        },
    )
    .map_err(|e| bad(&format!("Invalid child XML: {e}")))?;
    let root = doc.root_element();
    if root.tag_name().name() != "svg" || root.tag_name().namespace() != Some(SVG_NS) {
        return Err(bad("Child must be an SVG document"));
    }
    let mut ids = BTreeMap::new();
    for n in doc.descendants().filter(|n| n.is_element()) {
        budget.check()?;
        if n.ancestors().take(65).count() > 64 {
            return Err(bad("Child SVG nesting exceeds 64"));
        }
        if n.tag_name().namespace() != Some(SVG_NS) || !ELEMENTS.contains(&n.tag_name().name()) {
            return Err(bad("Unsupported or active SVG element"));
        }
        if let Some(id) = n.attribute("id") {
            if id.is_empty() || ids.contains_key(id) {
                return Err(bad("Empty or duplicate child SVG ID"));
            }
            ids.insert(id.into(), format!("{prefix}-{}", ids.len()));
        }
    }
    fn emit(
        n: roxmltree::Node<'_, '_>,
        out: &mut String,
        ids: &BTreeMap<String, String>,
        budget: &Budget<'_>,
    ) -> Result<()> {
        budget.check()?;
        if n.is_text() {
            out.push_str(&xml(n.text().unwrap_or("")));
            return Ok(());
        }
        if !n.is_element() {
            return Ok(());
        }
        write!(out, "<{}", n.tag_name().name()).unwrap();
        for a in n.attributes() {
            if a.namespace().is_some()
                || (!ATTRS.contains(&a.name())
                    && !matches!(a.name(), "data-edge-id" | "data-label-kind"))
            {
                return Err(bad(&format!("Unsupported SVG attribute {}", a.name())));
            }
            let text = if a.name() == "id" {
                ids[a.value()].clone()
            } else if a.name() == "style" {
                let mut style = String::new();
                let mut seen = BTreeSet::new();
                for d in a.value().split(';').filter(|s| !s.trim().is_empty()) {
                    let (key, val) = d
                        .split_once(':')
                        .ok_or_else(|| bad("Malformed inline style"))?;
                    let key = key.trim();
                    if !CSS.contains(&key) || !seen.insert(key) {
                        return Err(bad("Unsupported or duplicate style property"));
                    }
                    write!(style, "{key}:{};", value(val.trim(), ids)?).unwrap();
                }
                style
            } else {
                value(a.value(), ids)?
            };
            write!(out, " {}=\"{}\"", a.name(), xml(&text)).unwrap();
        }
        out.push('>');
        for c in n.children() {
            emit(c, out, ids, budget)?;
        }
        write!(out, "</{}>", n.tag_name().name()).unwrap();
        if out.len() > MAX_FRAME_BYTES {
            return Err(bad("Composed child exceeds budget"));
        }
        Ok(())
    }
    let mut out = String::new();
    emit(root, &mut out, &ids, budget)?;
    Ok(out)
}
