use mermaid_rs_renderer::layout::DiagramData;
use mermaid_rs_renderer::*;
#[test]
fn sankey_reserves_both_facing_captions() {
    let g=parse_mermaid_strict("sankey-beta\nSource artifact with provenance,Validated dataset with revision,10\nValidated dataset with revision,Accepted result,10").unwrap().graph;
    let c = LayoutConfig::default();
    let t = Theme::modern();
    let l = compute_layout(&g, &t, &c);
    assert!(l.width > 1000.);
    let svg = render_svg(&l, &t, &c);
    assert!(svg.find("class=\"node-labels\"").unwrap() > svg.find("class=\"links\"").unwrap());
    assert!(svg.contains("font-family="));
}
#[test]
fn kanban_header_clears_first_card() {
    let g = parse_mermaid_strict("kanban\n todo[To Do]\n   card[First card]")
        .unwrap()
        .graph;
    let l = compute_layout(&g, &Theme::modern(), &LayoutConfig::default());
    let s = &l.subgraphs[0];
    let n = l.nodes.values().next().unwrap();
    assert!(n.y >= s.y + 12. + s.label_block.height + 6.);
}

#[test]
fn sankey_labels_keep_their_band_centres_when_space_exists() {
    let g = parse_mermaid_strict("sankey-beta\nSource,Accepted,9\nSource,Rejected,3")
        .unwrap()
        .graph;
    let t = Theme::modern();
    let c = LayoutConfig::default();
    let l = compute_layout(&g, &t, &c);
    let DiagramData::Sankey(ref sankey) = l.diagram else {
        panic!("Sankey expected")
    };
    let svg = render_svg(&l, &t, &c);
    for node in &sankey.nodes {
        let baseline = node.y + node.height / 2.0 - 14.0 * 0.4;
        assert!(
            svg.contains(&format!("y=\"{baseline:.2}\" dy=\"0em\"")),
            "{} lost its band centre",
            node.label
        );
    }
}
