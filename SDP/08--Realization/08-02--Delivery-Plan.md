---
document_id: SDP-08-02
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# Vertical delivery plan

## Increments

| Tier | Usable result | Completion evidence |
|---|---|---|
| BX-T01 | Study, vocabulary and one precise static BoxUI contract | Resolved first-profile decisions, examples/negative cases and ownership review. |
| BX-T02 | Parse, validate and render nested regions with basic widgets and an embedded diagram | End-to-end static fixtures, selected viewports, local child errors and renderer regressions. |
| BX-T03 | Host-mediated input, focus, editable draft and typed intent | Interactive witness including old-frame input, replacement and failed preparation. |
| BX-T04 | Contracted synthetic Activity scenario through the same bindings | Trace/assertions, declared simulated contributions, suspension/context-change alternatives. |
| BX-T05 | Optional separate XFMD stream and SDL/Analyzer adapters | Versioned cross-repository contracts and actual integration evidence. |

Tier IDs describe vertical delivery. They do not require a particular sprint
ritual or imply that every horizontal layer is finished first. BX-T05 can be
split; stream transport is not a prerequisite for the first in-process witness.

## Immediate next work

Study existing BOX/BoxUI and renderer interfaces, settle BX-T01's bounded profile
and complete missing acceptance values. Implement only after the affected
contracts explain how the change fits. Update evidence and source bindings with
each increment; keep blocked targets accurate.

