---
document_id: SDP-09-01
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# Verification plan

## Obligations and evidence

| Requirement scope | Required checks |
|---|---|
| BX-R01, BX-R09 | Profile/version discrimination, invalid syntax, supported build features and relevant existing-family regressions. |
| BX-R02, BX-R03 | Nested regions, measurement, viewport cases, long text, invalid/oversized child diagrams and clipping. |
| BX-R04, BX-R12 | Typed input/value mappings, keyboard/focus/editing, disabled/error state and host accessibility contract. |
| BX-R05, BX-R06, BX-R07 | Input during preparation, stale result, removed widget, retained draft, retry and coherent publication. |
| BX-R08 | Chosen size/depth/work budgets and observable bounded failure. |
| BX-R10 | Controlled fixtures/time, simulated-participant inventory, independent assertions and reproducible trace. |
| BX-R11 | Schema/identity/path/reference checks and honest module/target/blueprint readiness. |

For a claimed result record subject, requirement IDs, source/model revision,
environment and dependencies, exact check, observed outcome, artifacts and limits.
A planned test, `verifies` link or static SVG is not a passed interaction test.

## Current limits

Phase 046 has BoxUI product tests, a real static render witness and schema checks
against actual Rust protocol output. Outcomes are in
[the evidence record](09-02--Evidence.md). They cover the renderer subset;
native XFMD input/publication/export, networking and production correctness
are not established by these checks.
An independent review of implementation can be planned with each Tier; this
document does not imply it has occurred.
