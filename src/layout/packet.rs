use super::*;

/// Proportional bit grid with measured endpoints and per-row overflow legends.
pub(super) fn compute(graph: &Graph, theme: &Theme, config: &LayoutConfig) -> Layout {
    let mut nodes = BTreeMap::new();
    let last_bit = graph.packet_fields.iter().map(|f| f.end).max().unwrap_or(0);
    let measure = |text: &str| {
        measure_label_with_font_size(text, theme.font_size, config, false, &theme.font_family)
    };
    // SVG uses the theme font for these Text nodes too. Reserve the widest actual
    // endpoint/marker, not a smaller-font approximation or an arbitrary 20px box.
    let mut bit = 26f32;
    let mut endpoint_height = 0f32;
    for field in &graph.packet_fields {
        for value in [field.start, field.end] {
            let block = measure(&value.to_string());
            bit = bit.max(block.width + 8.);
            endpoint_height = endpoint_height.max(block.height);
        }
    }
    // Row splits introduce endpoints not explicitly present in the input.
    for row in 0..=last_bit / 32 {
        for value in [row * 32, row * 32 + 31] {
            let block = measure(&value.to_string());
            bit = bit.max(block.width + 8.);
            endpoint_height = endpoint_height.max(block.height);
        }
    }
    for index in 0..graph.packet_fields.len() {
        bit = bit.max(measure(&format!("[{}]", index + 1)).width + 10.);
    }
    let grid_width = 32. * bit;
    let mut scene_width = grid_width + 48.;
    let mut y = 24.;
    for row in 0..=last_bit / 32 {
        measurements::checkpoint();
        let mut cells = vec![];
        let mut legends = vec![];
        let mut height = 48f32;
        for (index, f) in graph.packet_fields.iter().enumerate() {
            let start = f.start.max(row * 32);
            let end = f.end.min(row * 32 + 31);
            if start > end {
                continue;
            }
            let width = (end - start + 1) as f32 * bit;
            let mut block = measure_label_with_max_width(
                &f.label,
                theme.font_size,
                width - 10.,
                config,
                true,
                &theme.font_family,
            );
            if block.width > width - 10. {
                let marker = format!("[{}]", index + 1);
                let range = if start == end {
                    start.to_string()
                } else {
                    format!("{start}-{end}")
                };
                let text = format!("{marker} Bits {range}: {}", f.label);
                let label = measure_label_with_max_width(
                    &text,
                    theme.font_size,
                    grid_width,
                    config,
                    true,
                    &theme.font_family,
                );
                legends.push((index, label));
                block = measure(&marker);
            }
            height = height.max(block.height + 16.);
            cells.push((index, start, end, width, block));
        }
        let cell_y = y + endpoint_height + 8.;
        for (index, start, end, width, label) in cells {
            let x = 24. + (start % 32) as f32 * bit;
            insert_node(
                &mut nodes,
                format!("packet_{index}_{row}"),
                x,
                cell_y,
                width,
                height,
                label,
                crate::ir::NodeShape::Rectangle,
            );
            for (suffix, value, center) in [
                ("start", start, x + bit / 2.),
                ("end", end, x + width - bit / 2.),
            ] {
                if suffix == "end" && start == end {
                    continue;
                }
                let label = measure(&value.to_string());
                insert_node(
                    &mut nodes,
                    format!("bit_{index}_{row}_{suffix}"),
                    center - label.width / 2.,
                    y,
                    label.width,
                    endpoint_height,
                    label,
                    crate::ir::NodeShape::Text,
                );
            }
        }
        y = cell_y + height + 8.;
        for (index, label) in legends {
            // Generic SVG text is centered in its node. Size the node to the
            // measured text so all legends have the same left edge, unclipped.
            let width = label.width;
            let height = label.height;
            scene_width = scene_width.max(width + 48.);
            insert_node(
                &mut nodes,
                format!("packet_legend_{index}_{row}"),
                24.,
                y,
                width,
                height,
                label,
                crate::ir::NodeShape::Text,
            );
            y += height + 8.;
        }
        y += 16.;
    }
    Layout {
        kind: graph.kind,
        nodes,
        edges: vec![],
        subgraphs: vec![],
        width: scene_width,
        height: y,
        diagram: DiagramData::Graph {
            state_notes: vec![],
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn insert_node(
    nodes: &mut BTreeMap<String, NodeLayout>,
    id: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: TextBlock,
    shape: crate::ir::NodeShape,
) {
    let style = if shape == crate::ir::NodeShape::Text {
        crate::ir::NodeStyle {
            fill: Some("none".into()),
            stroke: Some("none".into()),
            ..Default::default()
        }
    } else {
        Default::default()
    };
    nodes.insert(
        id.clone(),
        NodeLayout {
            id,
            x,
            y,
            width,
            height,
            label,
            shape,
            style,
            link: None,
            anchor_subgraph: None,
            hidden: false,
            icon: None,
        },
    );
}
