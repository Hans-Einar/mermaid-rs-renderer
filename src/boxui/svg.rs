use super::{frame::*, model::*};
use anyhow::{Result, ensure};
use std::collections::BTreeMap;
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
fn text(x: f64, y: f64, s: &str) -> String {
    format!(
        "<text x=\"{x}\" y=\"{y}\" fill=\"#0F172A\" font-family=\"DejaVu Sans\" font-size=\"16\">{}</text>",
        escape(s)
    )
}
fn rect(r: Rect) -> String {
    format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#F8FAFC\" stroke=\"#94A3B8\"/>",
        r.x, r.y, r.width, r.height
    )
}
pub(super) fn document(w: f64, h: f64, body: &str, simulated: bool) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\"><rect width=\"{w}\" height=\"{h}\" fill=\"#FFFFFF\"/>{}{body}</svg>",
        text(
            8.,
            18.,
            if simulated {
                "BoxUI — simulated session"
            } else {
                "BoxUI — unbound (enable prototype explicitly)"
            }
        )
    )
}
pub(super) fn widget(
    n: &Node,
    r: Rect,
    value: &str,
    enabled: bool,
    children: &BTreeMap<String, ChildScene>,
    diagnostics: &mut Vec<String>,
    measure: &dyn Fn(&str) -> (f64, f64),
) -> Result<(String, String)> {
    let label = n.label.as_deref().unwrap_or("");
    let mut body = String::new();
    let mut edit = None;
    match n.kind {
        Kind::Text => {
            let mut line = String::new();
            let mut y = r.y + 18.;
            for word in n.text.as_deref().unwrap_or("").split_whitespace() {
                let candidate = if line.is_empty() {
                    word.into()
                } else {
                    format!("{line} {word}")
                };
                if !line.is_empty() && measure(&candidate).0 > r.width {
                    body += &text(r.x, y, &line);
                    line.clear();
                    y += 20.;
                }
                if !line.is_empty() {
                    line.push(' ');
                }
                line += word;
            }
            body += &text(r.x, y, &line);
        }
        Kind::Value => {
            body += &text(r.x, r.y + 17., label);
            body += &text(r.x, r.y + 39., value);
        }
        Kind::Button => {
            body += &rect(r);
            body += &text(r.x + 8., r.y + 24., label);
            if !enabled {
                body = format!("<g opacity=\"0.5\">{body}</g>");
            }
        }
        Kind::Input => {
            body += &text(r.x, r.y + 18., label);
            let field = Rect {
                x: r.x,
                y: r.y + 24.,
                width: r.width,
                height: 32.,
            };
            body += &rect(field);
            edit = Some(body.clone());
            body += &text(r.x + 6., r.y + 46., value);
        }
        Kind::Diagram => {
            body += &rect(r);
            if let Some(child) = children.get(&n.id) {
                if !child.error.is_empty() {
                    diagnostics.push(format!("{}: {}", n.id, child.error));
                    body += &text(r.x + 4., r.y + 20., &child.error);
                } else {
                    ensure!(
                        child.width.is_finite()
                            && child.height.is_finite()
                            && child.width > 0.
                            && child.height > 0.,
                        "Invalid child extent"
                    );
                    ensure!(child.svg.len() <= 8 * 1024 * 1024, "Child SVG too large");
                    let lower = child.svg.to_lowercase();
                    ensure!(
                        ![
                            "<script",
                            "<!doctype",
                            "<foreignobject",
                            "http://",
                            "https://"
                        ]
                        .iter()
                        .any(|v| lower
                            .replace("http://www.w3.org/2000/svg", "")
                            .replace("http://www.w3.org/1999/xlink", "")
                            .contains(v)),
                        "Unsafe child SVG"
                    );
                    let scale = ((r.width - 8.) / child.width).min((r.height - 8.) / child.height);
                    let start = child
                        .svg
                        .find("<svg")
                        .ok_or_else(|| anyhow::anyhow!("Invalid child SVG"))?;
                    let mut svg = child.svg[start..].to_string();
                    // Namespace IDs from library-emitted SVG, including marker/clip references.
                    let re = regex::Regex::new(r#"\bid="([^"]+)""#)?;
                    let ids: Vec<String> = re.captures_iter(&svg).map(|c| c[1].into()).collect();
                    for id in ids {
                        let prefix = format!("bx-{}-{id}", n.id);
                        svg = svg
                            .replace(&format!("id=\"{id}\""), &format!("id=\"{prefix}\""))
                            .replace(&format!("url(#{id})"), &format!("url(#{prefix})"))
                            .replace(&format!("href=\"#{id}\""), &format!("href=\"#{prefix}\""));
                    }
                    body += &format!(
                        "<g transform=\"translate({},{}) scale({scale})\">{svg}</g>",
                        r.x + (r.width - child.width * scale) / 2.,
                        r.y + (r.height - child.height * scale) / 2.
                    );
                }
            } else {
                let e = format!("{}: child not prepared", n.id);
                diagnostics.push(e.clone());
                body += &text(r.x + 4., r.y + 20., &e);
            }
        }
        _ => {}
    }
    let clip = format!(
        "<clipPath id=\"bx-clip-{}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath>",
        n.id, r.x, r.y, r.width, r.height
    );
    let wrap = |s: String| {
        format!(
            "<defs>{clip}</defs><g clip-path=\"url(#bx-clip-{})\">{s}</g>",
            n.id
        )
    };
    Ok((wrap(body.clone()), wrap(edit.unwrap_or(body))))
}
