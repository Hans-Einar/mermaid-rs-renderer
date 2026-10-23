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
library, and this pilot for the new **System Development Process**. Phase 046
implements the renderer-side BoxUI subset on `phase/boxui-046-implementation`,
in the isolated sibling worktree `mermaid-rs-renderer-boxui-implementation`.
The capture identifiers above are historical. SDL execution, native XFMD
integration and an Analyzer adapter are separate work.

## Start here

1. [Mandate](../01--Mandate/01-01--Mandate.md) defines scope and outcomes.
2. [Structure profile](00-02--SDP-Structure.md) defines reusable document slots.
3. [Discovery contract](00-03--Discovery-and-Tooling-Contract.md) defines the
   machine entry point, inventories, path rules and readiness.
4. [Project manifest](../sdp-project.json) enumerates documents, modules, targets,
   model entry points and blueprint descriptors.
5. [Verification evidence](../09--Verification/09-02--Evidence.md) distinguishes
   authoring evidence from implementation verification.
6. [Renderer API and XFMD handoff](../08--Realization/08-03--Renderer-API-and-XFMD-Handoff.md)
   gives the concrete phase-046 symbols, commands and remaining host work.

Document IDs are stable slots qualified by project ID. A filename, status label
or diagram is not proof of behavior. The manifest resolves each ID to one
canonical document; the mandate uses the standard numbered location.

## Current discovery summary

- Renderer Rust crate and additive BoxUI module: present; test evidence recorded.
- BoxUI parser/model/layout/widgets: implemented bounded subset; native interaction belongs to XFMD.
- SDL structural source: BoxUi.design, design-core 0.1. Executable scenario sources: none.
- Renderer/BoxUI test targets and standalone render witness: bound.
- Interactive run, SDL execution and blueprint generation targets: blocked as described in the manifest.
- SDP-Analyzer: future adapter required for this pilot profile.
