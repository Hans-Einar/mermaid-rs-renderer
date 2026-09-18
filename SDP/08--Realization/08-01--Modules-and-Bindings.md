---
document_id: SDP-08-01
profile: sdp-system-development-pilot/0.1
status: inventory
updated: 2026-09-18
---

# Modules and bindings

## Canonical inventory

The module and target lists live in [sdp-project.json](../sdp-project.json).
They identify existing renderer/BoxUI code and planned host integration components.
Do not maintain a second authoritative list here.

The existing Rust library test target records `cargo test --locked --lib`
at repository root. Phase 046 ran the library tests and recorded corpus/profile
results in SDP-09-02; toolchain/dependency readiness is still checked when invoked.

Phase 046 binds the BoxUI conformance test and standalone render witness. The
[renderer API handoff](08-03--Renderer-API-and-XFMD-Handoff.md) maps concrete
symbols to host responsibilities. Interactive host and SDL execution targets
retain their blockers. The scoped design-artifact checker remains a separate
authoring target. One structural design-core 0.1 source set and entry point is declared;
its parser was run from the inspected sibling SDP checkout. No portable parser
runner or executable SDL behavior is bound here. Generated outputs are not sources.

## Binding completion

A binding records model identity, binding role, repository/revision, path and
qualified symbol or IR entry, with adapter/profile versions and evidence.
Classify interpreted, native, simulated and missing contributions explicitly.
Dependencies and dynamic routing must be traced; a suggestive symbol name is
not proof of behavioral coverage.
