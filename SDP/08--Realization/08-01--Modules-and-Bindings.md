---
document_id: SDP-08-01
profile: sdp-system-development-pilot/0.1
status: inventory
updated: 2026-09-18
---

# Modules and bindings

## Canonical inventory

The module and target lists live in [sdp-project.json](../sdp-project.json).
They identify existing renderer code and planned BoxUI/integration components.
Do not maintain a second authoritative list here.

The existing Rust library test target records `cargo test --locked --lib`
at repository root. It was not run for this documentation-only delivery;
toolchain/dependency readiness is checked when execution is requested.

All BoxUI compile/run/test targets have no runner binding yet. They must be listed
with blockers rather than hidden or marked runnable. No SDL source set/entry
point is currently declared. Generated outputs are not sources.

## Binding completion

A binding records model identity, binding role, repository/revision, path and
qualified symbol or IR entry, with adapter/profile versions and evidence.
Classify interpreted, native, simulated and missing contributions explicitly.
Dependencies and dynamic routing must be traced; a suggestive symbol name is
not proof of behavioral coverage.

