---
document_id: SDP-05-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# System boundaries and channels

## Candidate boundary view

The renderer crate is a **library module**, not automatically a running Container.
BoxUI extends that library. A host such as XFMD owns application/window lifecycle
and input integration. A scenario runner or SDL interpreter owns modeled domain
state and may run in-process or separately; deployment is not selected yet.

Potential contracts:
- Authoring input -> typed BoxUI model and diagnostics.
- Model, values, viewport and theme -> prepared visual/interaction output.
- Host input -> typed intent with widget/frame identity.
- Interpreter -> Presentation values/results through declared bindings.
- Optional host stream -> staged content and atomic publication.

These are boundary responsibilities, not selected transports or OS threads.
BoxUI inputs/outputs must not replace all domain Channels with a single UI pipe.

## Completion questions

Choose ownership for text edit state, focus, event correlation, diagram caches
and frame lifetime. Define the minimum host contract and error isolation.
Assess compatibility of existing parse/layout/render APIs and scene consumers.
Trace BX-R01 through BX-R12 to relevant boundaries before finalizing allocation.

