---
document_id: SDP-03-02
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Requirements

## Status and classification

The following are derived requirements within the owner-accepted draft1 scope
(BX-D15).
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
and verification documents map contributions and checks. Draft1 now supplies proposed BX-R08 budgets, BX-R02 viewports and BX-R12
input/accessibility scope below. BX-D15 accepted this design baseline; real
implementation evidence is tracked separately. Specified values are not measured
product results.

Prototype checks do not establish native production-toolkit equivalence or
physical machine suitability.


## Derived XFMD obligations and acceptance values — draft1

| ID | Nature | Obligation | Acceptance |
|---|---|---|---|
| BX-R13 | Functional | XFMD is the required native interactive host for BoxUI in mixed Markdown. | BX-AT01/04/06: no browser process, real FOX input, unaffected adjacent text/links. |
| BX-R14 | Constraint | Preview/PDF share geometry; PDF freezes accepted values and never runs interactions. | BX-AT08: vector export, simulated marker, no draft leakage or dispatch. |
| BX-R15 | Constraint | Separate renderer/host workstreams share a versioned contract before parallel implementation. | BX-AT12: registered artifacts, ownership, independently usable mocks, review gate. |

BX-R08 numeric budgets are in host contract section 8. BX-R02 test viewports are
320x240, 640x480 and 1280x720, with overflow expected when minima cannot fit.
BX-R12 requires keyboard-only activation, focus traversal, Unicode editing and
clipboard; role/name/disabled descriptors. Screen-reader integration and broad
IME/platform equivalence are explicitly not established by the prototype.
[Acceptance catalog](../09--Verification/09-03--Acceptance-Catalog.md) gives cases
and expected outcomes. Accepted obligations are not automatically passed product tests.
