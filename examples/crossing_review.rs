//! Isolated presentation fixture: fixed routes with one proper crossing.
use mermaid_rs_renderer::render::{CrossingJumps, render_svg_with_crossings};
use mermaid_rs_renderer::{LayoutConfig, Theme, compute_layout, parse_mermaid_strict, render_svg};
fn main() -> anyhow::Result<()> {
    let prefix = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("usage: crossing_review OUTPUT_PREFIX"))?;
    let graph =
        parse_mermaid_strict("flowchart TD\n A[West] --> B[East]\n C[North] --> D[South]")?.graph;
    let theme = Theme::modern();
    let config = LayoutConfig::default();
    let mut l = compute_layout(&graph, &theme, &config);
    for (id, x, y) in [
        ("A", 10., 140.),
        ("B", 410., 140.),
        ("C", 210., 10.),
        ("D", 210., 270.),
    ] {
        let n = l.nodes.get_mut(id).unwrap();
        n.x = x;
        n.y = y;
        n.width = 80.;
        n.height = 40.;
    }
    l.edges[0].points = vec![(90., 160.), (410., 160.)];
    l.edges[1].points = vec![(250., 50.), (250., 270.)];
    l.width = 500.;
    l.height = 320.;
    std::fs::write(
        format!("{prefix}-plain.svg"),
        render_svg(&l, &theme, &config),
    )?;
    std::fs::write(
        format!("{prefix}-jumps.svg"),
        render_svg_with_crossings(&l, &theme, &config, CrossingJumps::default()),
    )?;
    Ok(())
}
