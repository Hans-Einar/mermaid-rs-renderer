use mermaid_rs_renderer::layout::{
    compute_layout_with_metrics,
    routed::{self, Engine, quality},
};
use mermaid_rs_renderer::render::{CrossingJumps, render_svg_with_crossings};
use mermaid_rs_renderer::routing_backend::RoutingControl;
use mermaid_rs_renderer::{LayoutConfig, Theme, parse_mermaid_strict, render_svg};
use std::time::{Duration, Instant};
fn framed(svg: String, b: (f32, f32, f32, f32)) -> String {
    let inner = &svg[svg.find('>').unwrap() + 1..svg.rfind("</svg>").unwrap()];
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"{} {} {} {}\">{inner}</svg>",
        b.2 - b.0,
        b.3 - b.1,
        b.0,
        b.1,
        b.2 - b.0,
        b.3 - b.1
    )
}
fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    anyhow::ensure!(
        args.len() >= 3,
        "usage: libavoid_review INPUT OUTPUT_PREFIX [fixed]"
    );
    let graph = parse_mermaid_strict(&std::fs::read_to_string(&args[1])?)?.graph;
    let theme = Theme::modern();
    let config = LayoutConfig::default();
    let start = Instant::now();
    let (legacy, stages) = compute_layout_with_metrics(&graph, &theme, &config);
    let old_time = start.elapsed();
    let control = RoutingControl {
        deadline: Instant::now() + Duration::from_secs(60),
        cancelled: &|| false,
    };
    let fixed = args.get(3).is_some_and(|s| s == "fixed");
    let result = if fixed {
        routed::route_positioned(&legacy, &control)
    } else {
        routed::compute(&graph, &theme, &config, Engine::Libavoid, &control)
    };
    let result = match result {
        Ok(r) => r,
        Err(e) => {
            let error = serde_json::json!({"status":"error","error":e.to_string(),"legacy":quality::measure(&legacy,8.)});
            std::fs::write(
                format!("{}-metrics.json", args[2]),
                serde_json::to_string_pretty(&error)?,
            )?;
            return Err(e.into());
        }
    };
    let metrics = serde_json::json!({"mode":if fixed{"fixed positions and text"}else{"end to end"},"legacy":quality::measure(&legacy,8.),"libavoid":quality::measure(&result.layout,8.),"legacy_total_ms":old_time.as_secs_f64()*1000.,"legacy_route_ms":(stages.edge_routing_us+stages.port_assignment_us) as f64/1000.,"libavoid_route_ms":result.routing_time.as_secs_f64()*1000.,"libavoid_total_ms":result.total_time.as_secs_f64()*1000.,"diagnostics":result.diagnostics});
    println!("{}", serde_json::to_string_pretty(&metrics)?);
    std::fs::write(
        format!("{}-metrics.json", args[2]),
        serde_json::to_string_pretty(&metrics)?,
    )?;
    let mut bounds = (
        0_f32,
        0_f32,
        legacy.width.max(result.layout.width),
        legacy.height.max(result.layout.height),
    );
    for l in [&legacy, &result.layout] {
        for e in &l.edges {
            for &p in &e.points {
                bounds.0 = bounds.0.min(p.0 - 16.);
                bounds.1 = bounds.1.min(p.1 - 16.);
                bounds.2 = bounds.2.max(p.0 + 16.);
                bounds.3 = bounds.3.max(p.1 + 16.);
            }
            if let (Some(t), Some(p)) = (&e.label, e.label_anchor) {
                bounds.0 = bounds.0.min(p.0 - t.width / 2. - 16.);
                bounds.1 = bounds.1.min(p.1 - t.height / 2. - 16.);
                bounds.2 = bounds.2.max(p.0 + t.width / 2. + 16.);
                bounds.3 = bounds.3.max(p.1 + t.height / 2. + 16.);
            }
        }
    }
    std::fs::write(
        format!("{}-legacy.svg", args[2]),
        framed(render_svg(&legacy, &theme, &config), bounds),
    )?;
    std::fs::write(
        format!("{}.svg", args[2]),
        framed(render_svg(&result.layout, &theme, &config), bounds),
    )?;
    std::fs::write(
        format!("{}-jumps.svg", args[2]),
        framed(
            render_svg_with_crossings(&result.layout, &theme, &config, CrossingJumps::default()),
            bounds,
        ),
    )?;
    mermaid_rs_renderer::layout_dump::write_layout_dump(
        std::path::Path::new(&format!("{}.json", args[2])),
        &result.layout,
        &graph,
    )?;
    Ok(())
}
