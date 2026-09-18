//! Event-ordered vertical placement; reuses measured lanes, edges and note shapes.
//! Explicit starts/branches/ends preserve nested scope, including empty messages
//! around notes and activation boundaries. No parser or toolkit involvement.
use super::*;
use crate::ir::SequenceEvent;
pub(super) fn apply(graph: &Graph, layout: &mut Layout, theme: &Theme, config: &LayoutConfig) {
    let DiagramData::Sequence(data) = &mut layout.diagram else {
        return;
    };
    let text = |s: &str| {
        measure_label_with_font_size(s, theme.font_size, config, true, &theme.font_family)
    };
    let left = layout
        .nodes
        .values()
        .map(|n| n.x + n.width / 2.0)
        .fold(f32::INFINITY, f32::min);
    let right = layout
        .nodes
        .values()
        .map(|n| n.x + n.width / 2.0)
        .fold(f32::NEG_INFINITY, f32::max);
    let mut cursor = layout
        .nodes
        .values()
        .map(|n| n.y + n.height)
        .fold(0.0, f32::max)
        + 20.0;
    let mut max_depth = 0;
    let mut depth = 0usize;
    for event in &graph.sequence_events {
        match event {
            SequenceEvent::Start(..) => {
                depth += 1;
                max_depth = max_depth.max(depth)
            }
            SequenceEvent::End => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    let mut frames: Vec<SequenceFrameLayout> = vec![];
    let mut stack: Vec<usize> = vec![];
    let mut active: HashMap<String, Vec<f32>> = HashMap::new();
    data.activations.clear();
    for event in &graph.sequence_events {
        measurements::checkpoint();
        match event {
            SequenceEvent::Message(index) => {
                let edge = &mut layout.edges[*index];
                let height = edge.label.as_ref().map(|b| b.height).unwrap_or(0.0);
                let y = cursor + height + 12.0;
                let old = edge.points[0].1;
                for p in &mut edge.points {
                    p.1 += y - old;
                }
                let a = edge.points[0].0;
                let b = edge.points.last().unwrap().0;
                let label_width = edge.label.as_ref().map(|b| b.width).unwrap_or(0.0);
                edge.label_anchor = Some((
                    if a == b {
                        a + label_width / 2.0 + 42.0
                    } else {
                        (a + b) / 2.0
                    },
                    cursor + height / 2.0,
                ));
                cursor = edge.points.iter().map(|p| p.1).fold(y, f32::max) + 18.0;
            }
            SequenceEvent::Note(index) => {
                let note = &mut data.notes[*index];
                note.y = cursor;
                cursor += note.height + 18.0;
            }
            SequenceEvent::Activate(id) => {
                active.entry(id.clone()).or_default().push(cursor - 9.0);
            }
            SequenceEvent::Deactivate(id) => {
                if let Some(starts) = active.get_mut(id) {
                    if let Some(y) = starts.pop() {
                        let n = &layout.nodes[id];
                        data.activations.push(SequenceActivationLayout {
                            x: n.x + n.width / 2.0 - 5.0 + starts.len() as f32 * 6.0,
                            y,
                            width: 10.0,
                            height: (cursor - 9.0 - y).max(4.0),
                            participant: id.clone(),
                            depth: starts.len(),
                        });
                    }
                }
            }
            SequenceEvent::Start(kind, label) => {
                let keyword = match kind {
                    crate::ir::SequenceFrameKind::Alt => "alt",
                    crate::ir::SequenceFrameKind::Opt => "opt",
                    crate::ir::SequenceFrameKind::Loop => "loop",
                    crate::ir::SequenceFrameKind::Par => "par",
                    _ => "fragment",
                };
                let heading = text(keyword);
                let condition = text(&format!("[{label}]"));
                let pad = (max_depth - stack.len()) as f32 * 18.0;
                let x = left - pad;
                let heading_width = heading.width + 16.0;
                let width = (right - left + pad * 2.0).max(heading_width + condition.width + 24.0);
                let header_height = heading.height.max(condition.height) + 16.0;
                frames.push(SequenceFrameLayout {
                    kind: *kind,
                    x,
                    y: cursor,
                    width,
                    height: 0.0,
                    label_box: (x, cursor, heading_width, heading.height + 8.0),
                    label: SequenceLabel {
                        x: x + heading_width / 2.0,
                        y: cursor + heading.height / 2.0 + 4.0,
                        text: heading,
                    },
                    section_labels: vec![SequenceLabel {
                        x: x + heading_width + 12.0 + condition.width / 2.0,
                        y: cursor + condition.height / 2.0 + 4.0,
                        text: condition,
                    }],
                    dividers: vec![],
                });
                stack.push(frames.len() - 1);
                cursor += header_height;
            }
            SequenceEvent::Branch(label) => {
                let f = &mut frames[*stack.last().expect("validated event stack")];
                f.dividers.push(cursor);
                let heading = text(&format!("[{label}]"));
                f.width = f.width.max(heading.width + 24.0);
                let height = heading.height;
                f.section_labels.push(SequenceLabel {
                    x: f.x + 12.0 + heading.width / 2.0,
                    y: cursor + height / 2.0 + 8.0,
                    text: heading,
                });
                cursor += height + 24.0;
            }
            SequenceEvent::End => {
                let index = stack.pop().expect("validated event stack");
                frames[index].height = cursor - frames[index].y;
                cursor += 18.0;
            }
        }
    }
    // Containment includes nested frames and side notes, without changing order.
    for i in (0..frames.len()).rev() {
        let y = frames[i].y;
        let bottom = y + frames[i].height;
        let mut min_x = frames[i].x;
        let mut max_x = min_x + frames[i].width;
        for n in &data.notes {
            if n.y >= y && n.y + n.height <= bottom {
                min_x = min_x.min(n.x - 12.0);
                max_x = max_x.max(n.x + n.width + 12.0);
            }
        }
        for f in frames.iter().skip(i + 1) {
            if f.y > y && f.y + f.height < bottom {
                min_x = min_x.min(f.x - 12.0);
                max_x = max_x.max(f.x + f.width + 12.0);
            }
        }
        frames[i].x = min_x;
        frames[i].width = max_x - min_x;
    }
    data.frames = frames;
    for line in &mut data.lifelines {
        line.y2 = cursor;
    }
    for box_ in &mut data.footboxes {
        box_.y = cursor;
    }
    super::sequence::finalize_sequence_layout_bounds(layout);
}
