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
