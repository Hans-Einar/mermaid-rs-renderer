---
document_id: SDP-00-01
profile: sdp-system-development-pilot/0.1
status: pilot
updated: 2026-09-18
---

# Project index

## Identity and current position

Project: **Mermaid BoxUI extension**, project ID `mermaid-boxui`.
Worktree: `mermaid-rs-renderer-boxui`; branch at capture: `feature/boxui-extension`.
Inspected HEAD: `225a5a28632bbfd36c78fa522327f5dbe177d78e`.
The workspace note records integrated renderer baseline `afab5e982aead4d4cdec7c88455095733805ed36`.

The owner authorizes a distinct **boxui** extension in the renderer fork, a widget
library, and this pilot for the new **System Development Process**. The present
delivery establishes the mandate and document/discovery structure. It does not
implement BoxUI, an SDL compiler/interpreter or an Analyzer integration.

## Start here

1. [Mandate](../01--Mandate/01-01--Mandate.md) defines scope and outcomes.
2. [Structure profile](00-02--SDP-Structure.md) defines reusable document slots.
3. [Discovery contract](00-03--Discovery-and-Tooling-Contract.md) defines the
   machine entry point, inventories, path rules and readiness.
4. [Project manifest](../sdp-project.json) enumerates documents, modules, targets,
   model entry points and blueprint descriptors.
5. [Verification evidence](../09--Verification/09-02--Evidence.md) distinguishes
   this documentation delivery from future implementation verification.

Document IDs are stable slots qualified by project ID. A filename, status label
or diagram is not proof of behavior. The manifest resolves each ID to one
canonical document; an unnumbered mandate filename is an explicit pilot override.

## Current discovery summary

- Renderer Rust crate: present; inspected, not tested in this delivery.
- BoxUI parser/model/layout/widgets/interaction support: planned.
- SDL source entry points and executable sequence sources: none declared yet.
- Renderer library test target: existing Cargo invocation, not run here.
- BoxUI compile/run/test and blueprint generation targets: declared, blocked.
- SDP-Analyzer: future adapter required for this pilot profile.

