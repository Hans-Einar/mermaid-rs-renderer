# Bounded routing shortcuts: fork experiment

This branch evaluates a small change to the existing router before attempting a
new port-selection and routing algorithm. The upstream baseline is
[`3726ccb`](https://github.com/1jehuang/mermaid-rs-renderer/tree/3726ccbffe0e8032361eb9668694b24f77858060).
The fork is [Hans-Einar/mermaid-rs-renderer](https://github.com/Hans-Einar/mermaid-rs-renderer).
The default layout engine, node placement, parser, theme and SVG renderer are unchanged.

## Diagnosis

Upstream already optimizes route length and bends. The problem is partly the
interaction between stages with different priorities. Crossing reduction,
rectangle simplification and endpoint repairs can undo one another's choices.

Temporary stage tracing of the traceability fixture found, for example:

- `A → FN`: 483 units before cleanup, 442 after early cleanup, 730 after repairs.
- `I → CAP`: 391, 321 and 604 units respectively.
- `CH → I`: 349, 349 and 615 units respectively.

The original rectangle simplifier checked non-endpoint nodes but did not check
endpoint direction or endpoint reentry. Later passes repaired the resulting
geometry, sometimes with substantial detours. The instrumentation was removed;
no environment-controlled tracing remains in the library.

## Routing change

`src/layout/flowchart/path_cleanup.rs::simplify_flowchart_detour_rectangles`
retains the existing rectangle, shoulder and spine candidates. It also tries
replacing a span between existing path vertices with either orthogonal elbow.
It does not construct a new routing grid or recursively search port assignments.

A candidate is rejected if it:

- changes a selected port point or its terminal direction;
- increases length or bend count;
- leaves or enters an endpoint in the wrong direction;
- reenters an endpoint, crosses another node, or crosses a foreign subgraph.

Valid candidates use this bounded tradeoff in layout units:

```
cost = length + 24 × bends + 80 × crossings + 2 × collinear_overlap_length
```

A lower score must improve by more than one unit. Thus avoiding a crossing does
not justify an unlimited detour. This is a heuristic, not a claim of a globally
optimal route. All edges are considered when measuring crossings and overlap.

The new elbow search runs only for graphs with at most 64 edges and paths with
at most 32 vertices. There are at most `(P−1)(P−2)` elbow candidates per edge,
plus the existing candidate families. Each is checked against obstacles and
other segments; this is not constant-time overall. There is one sweep per call,
no recursion and no convergence loop. Paths over 64 vertices are left alone by
this simplifier. Self-loops are also left alone.

`post_route.rs` passes subgraph geometry into the simplifier. `edge_pipeline.rs`
invokes it again after endpoint repairs, before routing validation and edge-layout
construction, so the repair detours can be shortened without another routing
repair immediately undoing the result. Changed routes discard their old preferred
label anchors so the label placer can derive anchors from the final path.

## Label safety change

Visual inspection exposed a separate issue: the final own-edge label nudges
could move text into nodes or other labels after the main label placement had
finished. In `src/layout/label_placement.rs`, the small nudge now rejects node
obstacles, and the final larger move rejects both node/subgraph label obstacles
and other center-label rectangles. The final move also attempts to repair a
label already obscuring content, even when it is clear of its own edge.
The final search considers eight directions at four bounded scales. If no safe
candidate exists, the original position is retained. This does not redesign label placement or guarantee that
all text avoids all foreign edges.

## Reproduction

The review example writes the library's SVG directly, a JSON layout dump, timing
and `FlowchartQualityMetrics`. There is no XFMD conversion of diagram geometry.

```sh
cargo run --locked --profile release-fast --no-default-features \
  --example routing_review -- \
  tests/fixtures/flowchart/routing-review/traceability.mmd /tmp/traceability
rsvg-convert -o /tmp/traceability.png /tmp/traceability.svg
```

For a before/after comparison, the first branch commit contains only the example
and three review fixtures; it has the unmodified upstream router. Use identical
compiler settings, theme, fonts and configuration on both revisions. Local
exploration used `CARGO_PROFILE_RELEASE_FAST_LTO=false` and
`CARGO_PROFILE_RELEASE_FAST_OPT_LEVEL=1`. Timings from that profile are not release
performance promises.

## Visual comparison

Traceability, rendered directly by the library:

| Baseline | Fork experiment |
| --- | --- |
| [PNG](routing-shortcuts-preview/before.png) · [SVG](routing-shortcuts-preview/before.svg) | [PNG](routing-shortcuts-preview/after.png) · [SVG](routing-shortcuts-preview/after.svg) |

## Measured routing results

Final code revision: `3fb81308a032d128aef4b54cdc7506bd4bd6342f`.

Measured with the default configuration and modern theme. All seven diagrams
have zero reported bad exits/entries, endpoint intrusions/reentries and
non-endpoint node hits before and after. Center-to-center Manhattan totals are
unchanged apart from floating-point rounding, confirming the same node spacing.

| Fixture | Length before → after | Bends before → after | Crossings before → after |
| --- | ---: | ---: | ---: |
| `traceability` | 8506 → 5119 | 76 → 36 | 14 → 6 |
| `service-map` | 2471 → 1960 | 24 → 12 | 6 → 0 |
| `layer-delivery` | 5174 → 4990 | 10 → 10 | 0 → 2 |
| `complex` | 5983 → 5214 | 59 → 45 | 3 → 4 |
| `cycles` | 985 → 961 | 5 → 6 | 0 → 0 |
| `subgraph_direction` | 475 → 475 | 2 → 2 | 0 → 0 |
| `ports_arrow_pathing_regression` | 4121 → 3510 | 28 → 23 | 1 → 6 |

The traceability fixture improves by 39.8% in length and 52.6% in bends.
An earlier, more aggressive variant reached 47.4% and 69.7%, but changed
terminal directions required by existing branch and loopback tests. The final
version preserves those requirements. The regression threshold requires at
most 40 bends, 8 crossings and 5600 length units, while retaining every hard
geometry invariant.

**The result is mixed across diagrams.** The port regression fixture gains five
crossings (1 → 6), and its existing quality score worsens from 243 to 823.
`layer-delivery`, `complex` and `cycles` also have worse aggregate quality scores
despite shorter paths. This is not a uniform quality improvement and is not an
upstream-ready default change. The proposal to integrate the fork into XFMD
should remain open until these tradeoffs are resolved.

Raw measurements are in [routing-shortcuts-metrics.json](routing-shortcuts-metrics.json).
Visual review found that label attachment can still be ambiguous and foreign
edges can pass behind text. Routing metrics do not measure all of readability.

## Verification

Final focused validation passed **442 tests** (386 library + 28 layout +
8 invariant + 1 routing regression + 19 visual issue tests). It covers `layout_suite`,
`routing_shortcuts_suite`, `invariant_suite` and `visual_issue_suite`:

```sh
CARGO_PROFILE_RELEASE_FAST_LTO=false CARGO_PROFILE_RELEASE_FAST_OPT_LEVEL=1 \
cargo test --locked --profile release-fast --no-default-features --features cli \
  --lib --test layout_suite --test routing_shortcuts_suite \
  --test invariant_suite --test visual_issue_suite
cargo fmt -- --check
python3 scripts/check_repo.py
```

The invariant suite includes the repository fixture corpus and hard flowchart
routing checks. SVG and PNG previews were inspected for the review diagrams.
An earlier broader test pass exposed the endpoint and own-label regressions
that motivated the final guards; those assertions were retained. Optional PNG
feature builds, the full CI matrix and packaging have not been verified here.
Clippy could not run because this installation has no `cargo clippy` command.
No upstream PR has been opened.

## Scope and remaining work

This is an experimental improvement to the existing router. It does **not**
implement the proposed nearest-main-port search, adjacent-side combinations,
interval-halved subports, or a hard minimum gap between parallel routes. It can
trade additional crossings for substantially shorter and simpler paths. Node
placement and stale/nonlocal label preferences can still limit readability.

Before an upstream PR, evaluate a larger quality corpus, review crossing and
label tradeoffs, and decide whether the cost weights should be configurable.
The fork has not been integrated into or installed with XFMD.
