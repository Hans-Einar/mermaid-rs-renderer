use super::*;
/// Native domain layout followed by the established orthogonal router. Domain
/// kind, node sections, cardinalities and composite semantics are retained.
/// Composite state boundaries use their native router until group endpoints
/// can be represented without turning the whole group into a solid obstacle.
pub fn compute_semantic_layout(
    graph: &Graph,
    theme: &Theme,
    config: &LayoutConfig,
    control: &crate::routing_backend::RoutingControl<'_>,
) -> Result<(Layout, Vec<String>), crate::routing_backend::RoutingError> {
    use crate::ir::DiagramKind;
    if !matches!(
        graph.kind,
        DiagramKind::Class | DiagramKind::State | DiagramKind::Er | DiagramKind::Requirement
    ) {
        return Err(crate::routing_backend::RoutingError::InvalidInput(
            "not a semantic graph family".into(),
        ));
    }
    control.check()?;
    let mut selected = config.clone();
    selected.flowchart.auto_spacing.enabled = false;
    for attempt in 0..2 {
        let mut native = compute_layout(graph, theme, &selected);
        if !graph.subgraphs.is_empty() {
            return Ok((
                native,
                vec!["native composite state routing (group endpoints)".into()],
            ));
        }
        if graph.kind == DiagramKind::State {
            compact_empty_state_bands(
                &mut native,
                graph.direction,
                selected.rank_spacing.max(60.) * 2.,
            );
        }
        // Endpoint captions are placed on the completed routes, not treated as
        // center captions by the existing obstacle/attachment transaction.
        let endpoint_labels: Vec<_> = native
            .edges
            .iter_mut()
            .map(|e| (e.start_label.take(), e.end_label.take()))
            .collect();
        let mut routed = match routed::route_positioned(&native, control) {
            Ok(r) => r,
            Err(ref error)
                if attempt == 0
                    && (matches!(error, crate::routing_backend::RoutingError::NoSpace(_))
                        || matches!(error,crate::routing_backend::RoutingError::Backend(message) if message.starts_with("non-orthogonal edge"))) =>
            {
                selected.node_spacing = selected.node_spacing.max(50.) * 2. + 24.;
                selected.rank_spacing = selected.rank_spacing.max(50.) * 2. + 24.;
                continue;
            }
            Err(e) => return Err(e),
        };
        for (edge, (start, end)) in routed.layout.edges.iter_mut().zip(endpoint_labels) {
            edge.start_label = start;
            edge.end_label = end;
            edge.start_label_anchor = None;
            edge.end_label_anchor = None;
        }
        label_placement::resolve_endpoint_labels(
            &mut routed.layout.edges,
            &routed.layout.nodes,
            &routed.layout.subgraphs,
            None,
            routed.layout.kind,
            theme,
            config,
        );
        finalize_graph_label_bounds(&mut routed.layout, config);
        control.check()?;
        if attempt > 0 {
            routed.diagnostics.push("one semantic spacing retry".into());
        }
        return Ok((routed.layout, routed.diagnostics));
    }
    unreachable!()
}

// Cyclic native state ranks can leave very large empty bands. Compact only
// empty space, preserving rank order and all node extents before routing.
fn compact_empty_state_bands(layout: &mut Layout, direction: crate::ir::Direction, max_gap: f32) {
    let horizontal = matches!(
        direction,
        crate::ir::Direction::LeftRight | crate::ir::Direction::RightLeft
    );
    let mut intervals: Vec<_> = layout
        .nodes
        .values()
        .filter(|n| !n.hidden)
        .map(|n| {
            if horizontal {
                (n.x, n.x + n.width)
            } else {
                (n.y, n.y + n.height)
            }
        })
        .collect();
    intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut cuts = vec![];
    let mut end = intervals.first().map_or(0., |p| p.1);
    for (start, next) in intervals {
        if start - end > max_gap {
            cuts.push((start, start - end - max_gap));
        }
        end = end.max(next);
    }
    for n in layout.nodes.values_mut() {
        let p = if horizontal { n.x } else { n.y };
        let shift: f32 = cuts
            .iter()
            .filter(|(start, _)| *start <= p + 0.01)
            .map(|(_, delta)| delta)
            .sum();
        if horizontal {
            n.x -= shift;
        } else {
            n.y -= shift;
        }
    }
    layout.width = layout
        .nodes
        .values()
        .map(|n| n.x + n.width)
        .fold(0., f32::max)
        + 20.;
    layout.height = layout
        .nodes
        .values()
        .map(|n| n.y + n.height)
        .fold(0., f32::max)
        + 20.;
}
