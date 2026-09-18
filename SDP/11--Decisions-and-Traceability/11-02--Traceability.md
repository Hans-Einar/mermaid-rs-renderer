---
document_id: SDP-11-02
profile: sdp-system-development-pilot/0.1
status: pilot
updated: 2026-09-18
---

# Traceability and change impact

## Record roles

The manifest's relations link registered documents, modules, targets and
blueprints. Requirement, Feature, Use Case and Activity IDs currently appear in
their canonical numbered documents. Their structured semantic registry is not
yet implemented; a future adapter must report that distinction.

A requirement-to-document edge locates explanation, not proof of satisfaction.
An execution target with a command is not passed evidence.
Later model-derived relations should replace manual duplicates with stable
source references and schema-versioned records.

## Change impact

Trace need -> requirement -> functional responsibility/Activity -> allocation
and contract -> Function/widget binding -> source/IR -> check/evidence.
Trace backward when implementation reveals a missing condition or broken rule.

Include cross-cutting ownership, identity, viewport, lifecycle and compatibility
obligations. Keep accepted, proposed, implemented, observed and unknown facts
distinct. Do not use complete file counts as design-completion evidence.


## Draft1 tracing and change rules

Use Cases/requirements -> feature table -> named SDL Functionality -> detailed
function table -> contract section -> BX-ATxx. The structural SDL source preserves
ownership/capability facts only; requirement/temporal references remain in these
records. Binding/type change affects both parser and host; geometry change affects
layout, hit tests, overlay and PDF; session identity change affects ledger, drafts,
worker cancellation and synthetic oracle. Update both tasks before parallel edits.
Renderer bindings now exist for the bounded phase-046 subset below. Host-side
bindings remain separately owned and unverified here. No trace link alone proves
test completion.

## Phase 046 source and evidence mapping

| Responsibility / acceptance | Source realization | Concrete verification |
|---|---|---|
| ParseBoxUi / BX-AT02 | `src/boxui/parse.rs`, `model.rs`, `validate.rs` | `tests/boxui.rs`: shared model, UTF-8 byte ranges, invalid source/type/identity, boundary tests |
| DefineWidgets / BX-AT02 | `WidgetRegistry`, typed `Kind`/`DataType` | Unknown versions, forbidden properties and incompatible value/command tests |
| AllocateRegions / BX-AT03 | `src/boxui/layout.rs` | Deterministic geometry, wrapping/no-space, growth caps and invalid host metrics tests |
| PrepareFrame / BX-AT03/11 | `src/boxui/frame.rs`, `svg.rs`, `embedded.rs` | Coherent static/preview/control frame, inert child rejection, all three real child families, cancellation/budget/session tests |
| Contract preservation / BX-AT12 | `examples/boxui_host.rs`, `check_boxui_runtime.py` | Actual Rust wire output validated against unchanged draft1 schemas |
| Static visual witness / part of BX-AT08 | `examples/boxui_demo.rs` | Real state child, SVG rasterization and visual inspection; **not** XFMD PDF export evidence |

Remaining: BX-AT01/04–10 require XFMD behavior; BX-AT11 additionally requires
cross-repository pinning, native FFI ownership checks and sanitizer evidence.
The structural SDL facts and the host schemas were not expanded or redefined
to make implementation appear complete.
