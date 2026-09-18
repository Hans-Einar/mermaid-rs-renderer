---
document_id: SDP-03-02
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Requirements

## Status and classification

The following are derived candidate requirements within the authorized mandate.
They are not tested claims. Scope/perspective and functional/quality/constraint
nature are separate classifications. Changes retain IDs and rationale.

| ID | Nature | Obligation | Acceptance direction |
|---|---|---|---|
| BX-R01 | Functional | Recognize a distinct versioned BoxUI profile without changing existing Mermaid semantics. | Positive/negative parsing and existing-family regression fixtures. |
| BX-R02 | Functional | Preserve nested region/widget identity and declared layout constraints. | Same model at selected viewports; deterministic fixture expectations. |
| BX-R03 | Functional | Embed supported diagram content with defined scaling, clipping and local diagnostics. | Valid/invalid child content and sibling preservation. |
| BX-R04 | Functional | Bind typed widget input and values without moving domain authority into widgets. | Valid/invalid binding, input and stale-value cases. |
| BX-R05 | Functional | Preserve specified focus, selection and draft state across compatible presentation changes. | Interactive editing during replacement. |
| BX-R06 | Functional | Correlate input with its frame/binding revision and avoid duplicate committed intent. | Late input, retry, removed widget and reconnect cases. |
| BX-R07 | Constraint | Publish visuals, input geometry and bindings coherently; failed/obsolete rendering preserves the active frame. | Delayed, failed and superseded preparation. |
| BX-R08 | Quality | Bound parsing, nesting, embedded content and rendering work with explicit failure. | Numeric limits and budget cases to be chosen in detailed design. |
| BX-R09 | Constraint | Operate offline with declared dependencies and preserve existing renderer consumers. | Supported feature builds and compatibility checks. |
| BX-R10 | Functional | Distinguish simulated, implemented and missing behavior in prototype evidence. | Scenario trace with explicit binding inventory and independent assertions. |
| BX-R11 | Functional | Discover documents, modules, targets and blueprint descriptions from SDP root. | Manifest/schema/link checks including blocked targets. |
| BX-R12 | Functional | Support usable mouse/keyboard interaction and declared accessibility semantics. | Host witness for navigation, editing, disabled/error states. |

## Coverage and unresolved measurements

User cases BX-UC-01 through BX-UC-04 motivate these obligations. The functional
and verification documents map contributions and checks. BX-R08 budget values,
BX-R02 viewport cases and BX-R12 exact input/accessibility support remain open;
do not claim completed design or verification until relevant gaps are resolved.

Prototype checks do not establish native production-toolkit equivalence or
physical machine suitability.

