//! Standalone witness: interpretation of child Mermaid happens BEFORE BoxUI prepare.
use mermaid_rs_renderer::boxui::*;
use std::path::PathBuf;
fn main() -> anyhow::Result<()> {
    let directory = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/boxui-demo"));
    std::fs::create_dir_all(&directory)?;
    let source = format!(
        "boxui 0.1\n{}",
        include_str!("../SDP/09--Verification/fixtures/activity.boxui.json")
    );
    let parsed = parse_boxui(&source)?;
    let mut request = decode_prepare_json(include_bytes!(
        "../SDP/09--Verification/fixtures/prepare.json"
    ))?;
    request.model = parsed.model;
    request.children.clear();
    for child in parsed.child_sources {
        let svg = mermaid_rs_renderer::render(&child.source)?;
        let doc = roxmltree::Document::parse(&svg)?;
        let view: Vec<f64> = doc
            .root_element()
            .attribute("viewBox")
            .ok_or_else(|| anyhow::anyhow!("Missing child viewBox"))?
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        anyhow::ensure!(view.len() == 4, "Invalid child viewBox");
        request.children.push(PreparedChild::Svg {
            reference: child.reference,
            width: view[2],
            height: view[3],
            svg,
        });
    }
    let frame = prepare_boxui(&request, &MonospaceMetrics)?;
    anyhow::ensure!(
        frame.diagnostics.is_empty(),
        "Unexpected diagnostics: {:?}",
        frame.diagnostics
    );
    std::fs::write(
        directory.join("frame.json"),
        serde_json::to_vec_pretty(&frame)?,
    )?;
    std::fs::write(
        directory.join("prepare.json"),
        serde_json::to_vec_pretty(&request)?,
    )?;
    std::fs::write(directory.join("static.svg"), &frame.static_svg)?;
    std::fs::write(directory.join("preview.svg"), &frame.preview_svg)?;
    println!(
        "Prepared {} controls and a real Mermaid child in {}",
        frame.controls.len(),
        directory.display()
    );
    Ok(())
}
