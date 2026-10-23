use mermaid_rs_renderer::layout::{TextBlock, measurements};
use mermaid_rs_renderer::{
    Layout, LayoutConfig, Theme, compute_layout, parse_mermaid_strict, render_svg,
};
use std::time::Duration;

// Word-wrap measurer models XFMD's important contract: an unbreakable word may
// exceed the requested width. Layout must inspect returned dimensions.
fn measure(_: usize, text: &str, size: f32, _: f32, _: bool) -> TextBlock {
    TextBlock {
        lines: vec![text.into()],
        width: text.chars().count() as f32 * size * 0.6,
        height: size * 1.4,
    }
}
fn layout(source: &str, size: f32) -> Layout {
    let graph = parse_mermaid_strict(source).unwrap().graph;
    let mut theme = Theme::modern();
    theme.font_size = size;
    measurements::with_measurer(0, measure, Duration::from_secs(2), || {
        compute_layout(&graph, &theme, &LayoutConfig::default())
    })
}
fn assert_readable(l: &Layout) {
    for n in l.nodes.values() {
        assert!(n.label.width <= n.width + 0.01, "{} text exceeds box", n.id);
        assert!(
            n.label.height <= n.height + 0.01,
            "{} text exceeds height",
            n.id
        );
        assert!(
            n.x >= 0.
                && n.y >= 0.
                && n.x + n.width <= l.width + 0.01
                && n.y + n.height <= l.height + 0.01,
            "{} outside scene",
            n.id
        );
    }
    let numbers: Vec<_> = l
        .nodes
        .values()
        .filter(|n| n.id.starts_with("bit_"))
        .collect();
    for (i, a) in numbers.iter().enumerate() {
        for b in &numbers[i + 1..] {
            if (a.y - b.y).abs() < 0.01 {
                assert!(
                    a.x + a.width + 7.9 <= b.x || b.x + b.width + 7.9 <= a.x,
                    "{} overlaps {}",
                    a.id,
                    b.id
                );
            }
        }
    }
}
#[test]
fn n12_labels_and_boundaries_are_readable_without_changing_bit_ratios() {
    for size in [12., 16., 28.] {
        let l = layout(
            "packet\n0: \"PacketFlag\"\n1-12: \"PacketNumber\"\n13-28: \"PacketPayload\"",
            size,
        );
        assert_readable(&l);
        let flag = &l.nodes["packet_0_0"];
        assert_eq!(flag.label.lines, vec!["[1]"]);
        assert_eq!(
            l.nodes["packet_legend_0_0"].label.lines,
            vec!["[1] Bits 0: PacketFlag"]
        );
        assert!((l.nodes["packet_1_0"].width / flag.width - 12.).abs() < 0.001);
        assert!((l.nodes["packet_2_0"].width / flag.width - 16.).abs() < 0.001);
        assert!(
            l.nodes["bit_1_0_end"].x + l.nodes["bit_1_0_end"].width < l.nodes["bit_2_0_start"].x
        );
    }
}
#[test]
fn adjacent_single_bits_and_row_splits_keep_exact_ranges() {
    let l = layout(
        "packet\n+1: \"FirstLongFlag\"\n+1: \"SecondLongFlag\"\n+29: \"Body\"\n+2: \"CrossingLongName\"\n+1: \"LastFlag\"",
        16.,
    );
    assert_readable(&l);
    for (id, text) in [
        ("packet_legend_0_0", "[1] Bits 0: FirstLongFlag"),
        ("packet_legend_1_0", "[2] Bits 1: SecondLongFlag"),
        ("packet_legend_3_0", "[4] Bits 31: CrossingLongName"),
        ("packet_legend_3_1", "[4] Bits 32: CrossingLongName"),
    ] {
        assert_eq!(l.nodes[id].label.lines, vec![text]);
    }
    assert!(!l.nodes.contains_key("bit_0_0_end"));
    assert!(
        l.nodes["packet_3_1"].y
            > l.nodes["packet_legend_3_0"].y + l.nodes["packet_legend_3_0"].height
    );
}
#[test]
fn high_offsets_and_unbreakable_legend_expand_bounds() {
    let name = "Long".repeat(400);
    let source = format!("packet\n4093: \"{}\"\n4094: \"B\"\n4095: \"C\"", name);
    let l = layout(&source, 24.);
    assert_readable(&l);
    assert!(l.nodes["packet_legend_0_127"].label.lines[0].contains(&name));
    let svg = render_svg(&l, &Theme::modern(), &LayoutConfig::default());
    assert!(svg.contains(&name));
    assert!(!svg.contains("NaN"));
}
#[test]
fn default_font_measurement_preserves_regular_packet() {
    let g = parse_mermaid_strict("packet\n0-7: \"Type\"\n8-31: \"Revision\"\n+40: \"Payload\"")
        .unwrap()
        .graph;
    let l = compute_layout(&g, &Theme::modern(), &LayoutConfig::default());
    assert_readable(&l);
    assert!(l.nodes.contains_key("packet_2_2"));
}
