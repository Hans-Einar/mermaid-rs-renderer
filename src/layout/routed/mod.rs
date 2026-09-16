//! Explicit routing selection independent of the configured node placement engine.
mod geometry;
mod labels;
pub mod quality;
use super::Layout;
use crate::routing_backend::*;
use crate::{Graph, LayoutConfig, Theme, ir::DiagramKind};
use std::time::Instant;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Engine {
    Legacy,
    Libavoid,
}
#[derive(Debug)]
pub struct RoutedLayout {
    pub layout: Layout,
    pub engine: Engine,
    pub diagnostics: Vec<String>,
    pub routing_time: std::time::Duration,
    pub total_time: std::time::Duration,
}
/// Legacy is explicit. No automatic fallback hides a backend error.
pub fn compute(
    graph: &Graph,
    theme: &Theme,
    config: &LayoutConfig,
    engine: Engine,
    control: &RoutingControl<'_>,
) -> Result<RoutedLayout, RoutingError> {
    control.check()?;
    let start = Instant::now();
    if engine == Engine::Legacy {
        let (layout, metrics) = super::compute_layout_with_metrics(graph, theme, config);
        control.check()?;
        return Ok(RoutedLayout {
            layout,
            engine,
            diagnostics: vec![
                "Legacy routing: cancellation checked before/after layout only".into(),
            ],
            routing_time: std::time::Duration::from_micros(
                (metrics.edge_routing_us + metrics.port_assignment_us) as u64,
            ),
            total_time: start.elapsed(),
        });
    }
    if graph.kind != DiagramKind::Flowchart {
        return Err(RoutingError::InvalidInput(
            "alternative routing supports flowcharts only".into(),
        ));
    }
    let graph = super::normalize_graph_for_layout(graph);
    let layout = super::compute_flowchart_layout(&graph, theme, config, None, false);
    control.check()?;
    let mut result = route_positioned(&layout, control)?;
    // Only uniform translation/bounds adjustment after routing; never Legacy
    // route repair, label nudge, coordinate scaling or aspect folding.
    super::finalize_graph_label_bounds(&mut result.layout, config);
    result.total_time = start.elapsed();
    Ok(result)
}
/// Route an already placed/measured graph without moving its nodes. Used for
/// identical-position A/B comparisons and by the fresh-placement path above.
pub fn route_positioned(
    layout: &Layout,
    control: &RoutingControl<'_>,
) -> Result<RoutedLayout, RoutingError> {
    control.check()?;
    #[cfg(not(feature = "libavoid"))]
    {
        let _ = layout;
        Err(RoutingError::Unavailable)
    }
    #[cfg(feature = "libavoid")]
    {
        let start = Instant::now();
        let mut layout = layout.clone();
        let mut input = geometry::input(&layout)?;
        let mut diagnostics = vec![];
        let mut routing_time = std::time::Duration::ZERO;
        let mut completed = false;
        for pass in 0..3 {
            control.check()?;
            let output = Libavoid.route(&input, control)?;
            diagnostics.push(format!(
                "libavoid transaction {}: {:?}",
                pass + 1,
                output.elapsed
            ));
            routing_time += output.elapsed;
            for route in output.routes {
                let connection = &mut input.connections[route.id as usize];
                connection.source_port = Some(route.source_port);
                connection.target_port = Some(route.target_port);
                let edge = layout
                    .edges
                    .get_mut(route.id as usize)
                    .ok_or_else(|| RoutingError::Backend("unknown edge ID".into()))?;
                edge.points = route
                    .points
                    .iter()
                    .map(|p| (p.0 as f32, p.1 as f32))
                    .collect();
            }
            geometry::validate(&layout)?;
            if pass == 0 {
                labels::place(&mut layout)?;
                if labels::validate(&layout).is_ok() {
                    completed = true;
                    break;
                }
                input
                    .obstacles
                    .extend(labels::obstacles(&layout, input.obstacles.len() as u32 + 1));
                if layout.edges.iter().all(|e| e.label.is_none()) {
                    completed = true;
                    break;
                }
            } else if labels::validate(&layout).is_ok() {
                completed = true;
                break;
            } else if pass == 1 {
                labels::place(&mut layout)?;
                let connections = input.connections;
                input = geometry::input(&layout)?;
                input.connections = connections;
                input
                    .obstacles
                    .extend(labels::obstacles(&layout, input.obstacles.len() as u32 + 1));
            }
        }
        if !completed {
            labels::validate(&layout)?;
        }
        control.check()?;
        let quality = quality::measure(&layout, input.separation);
        if quality.parallel_overlaps > 0 || quality.close_parallel_segments > 0 {
            diagnostics.push(format!("constrained parallel separation: {quality:?}"));
        }
        Ok(RoutedLayout {
            layout,
            engine: Engine::Libavoid,
            diagnostics,
            routing_time,
            total_time: start.elapsed(),
        })
    }
}
