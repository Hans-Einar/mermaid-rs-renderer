use anyhow::{Context, ensure};
use mermaid_rs_renderer::layout::flowchart_quality_metrics;
use mermaid_rs_renderer::layout_dump::write_layout_dump;
use mermaid_rs_renderer::{LayoutConfig, Theme, compute_layout, parse_mermaid_strict, render_svg};
use std::{path::Path, time::Instant};
fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    ensure!(
        args.len() == 3,
        "usage: routing_review INPUT.mmd OUTPUT_PREFIX"
    );
    let source = std::fs::read_to_string(&args[1]).context("read Mermaid input")?;
    let graph = parse_mermaid_strict(&source)?.graph;
    let config = LayoutConfig::default();
    let theme = Theme::modern();
    let start = Instant::now();
    let layout = compute_layout(&graph, &theme, &config);
    println!(
        "elapsed_ms={} metrics={:?}",
        start.elapsed().as_millis(),
        flowchart_quality_metrics(&layout)
    );
    std::fs::write(
        format!("{}.svg", &args[2]),
        render_svg(&layout, &theme, &config),
    )?;
    write_layout_dump(Path::new(&format!("{}.json", &args[2])), &layout, &graph)?;
    Ok(())
}
