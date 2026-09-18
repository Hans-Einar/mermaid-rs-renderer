---
document_id: SDP-07-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Units, functions and data

## SDL source and coverage

[BoxUi.design](SDL/BoxUi.design) is an actual design-core 0.1 structural source:
one namespace, Units/Container, owned Functionalities, capabilities, interfaces,
Activities and mode-scoped dependency. The existing SDP parser checks it. It is
not an invented superset and does not encode runtime widgets as flowchart nodes.

It intentionally does not encode guards, Value types, event payloads, requirements,
source bindings or temporal behavior: current design-core cannot preserve these.
The following contracts and [host contract](../06--Container-Design/06-02--XFMD-Host-Contract.md)
carry that detail as reviewed prose/schema. No behavioral SDL compiler is claimed.
A later language extension must preserve this distinction and have its own tests.

## Function contracts and realization

| SDL Functionality / proposed function | Preconditions -> guarantees / failure |
|---|---|
| ParseBoxUi / parse_boxui | bounded UTF-8 + exact version -> immutable typed tree, byte spans; rejects duplicate IDs/keys, unsupported kinds or mismatched bindings atomically |
| AllocateRegions / layout_boxui | validated tree + finite viewport + metrics -> deterministic nonnegative geometry and clip tree; no-space returns diagnostic, never a negative or NaN box |
| DefineWidgets / WidgetRegistry | fixed built-in kind/version registry -> unsupported widgets reject; no runtime registration or plugin loading in 0.1 |
| PrepareFrame / prepare_boxui | tree + complete snapshot/key + fonts -> coherent SVG/control map; cancellation/child failure is explicit |
| PublishFrame / BoxUiCoordinator::publish | current sealed key + hidden overlays ready -> GUI-thread swap; obsolete/failure preserves old frame, input gating follows host contract |
| RouteIntent / BoxUiCoordinator::dispatch | visible event and current binding/context -> bounded once-per-command local dispatch or reason; no domain authority implied |
| RetainDraft / FoxBoxUiOverlay::reconcile | old/new identity compatibility -> retained draft/caret/focus or cancellation notice; never auto-submit |
| SimulateActivity / SyntheticActivity::accept | registered synthetic context + typed command -> recorded accepted/rejected/pending outcome; no production endpoint |

Phase 046 realizes `parse_boxui`, `layout_boxui`, `WidgetRegistry` and
`prepare_boxui` under `src/boxui/`; exact signatures and limitations are in
[SDP-08-03](../08--Realization/08-03--Renderer-API-and-XFMD-Handoff.md).
The coordinator, overlay and synthetic participant names remain host-side role
allocations, not verified installed symbols. Library/host allocation is separate
from SDL containment; BoxUiLibrary remains a library. No C++ class
or Rust helper gets an SDL object solely because it exists.

## Data lifetime

Authoring tree persists for a parsed source revision; snapshots are immutable.
Prepared frames persist through paint/input capture. Session ledger and per-control
drafts belong to the application and are destroyed on close/reload. Export owns a
frozen accepted-value snapshot. No pointers to temporary frame strings survive
result release. Child cache key includes content, supported profile, palette,
fonts and allocated bounds; it contains no live domain binding.
