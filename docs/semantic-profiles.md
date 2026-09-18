# Native domain layouts with orthogonal routing

`layout::compute_semantic_layout` retains Class, State, ER and Requirement kinds
and their existing native node geometry. For flat graphs it reuses
`routed::route_positioned`, including attached-label obstacles and diagnostics.
Endpoint labels are restored and placed on the final routes. One spacing retry
is allowed on no-space or non-orthogonal backend output; errors remain explicit.
Composite states retain native group-boundary routing, reported in diagnostics.

Cyclic state ranks may have excessive empty bands. A bounded compaction removes
only empty primary-axis space before routing, preserving node extents and order.
Class marker tips now meet node boundaries, with outward-facing hollow triangles
and readable multiplicities. ER cardinality glyphs extend outside both endpoints.

Self-loop free endpoints are moved just outside libavoid's shape buffer for the
transaction. The adapter restores short normal segments to the selected boundary
ports; libavoid still solves the obstacle-avoiding exterior connection. This fixes
locked loop endpoints becoming trapped inside their own inflated obstacle during
label rerouting. All emitted geometry goes through existing route validation.

`measurements::with_measurer` supplies a synchronous, scoped callback for actual
text metrics. Context never survives the operation; TLS restores on unwinding.
External consumers must preserve structured member lines when measuring class/ER
labels, since those lines still feed the library's native table rendering.

Validation: 398 library tests, 12 libavoid integration tests and three semantic
profile tests pass. `examples/semantic_svg.rs` renders bounded native fixtures to
stdout; it does not replace XFMD's stricter parser profile or measured-font tests.
