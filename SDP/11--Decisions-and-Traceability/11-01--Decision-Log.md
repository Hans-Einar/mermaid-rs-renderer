---
document_id: SDP-11-01
profile: sdp-system-development-pilot/0.1
status: pilot
updated: 2026-09-18
---

# Decision log

## Initial dispositions

| ID | Position | Authority/status | Consequence |
|---|---|---|---|
| BX-D01 | Name the custom extension BoxUI and extend our renderer fork. | Explicit owner direction, 2026-09-18. | Standard treemap remains unchanged; define a separate versioned profile. |
| BX-D02 | Add a reusable widget library and support embedded diagram content. | Explicit owner direction. | Specify semantic/input/host contracts, not only shapes. |
| BX-D03 | Pilot numbered standard SDP documents in this project. | Explicit owner direction; concrete structure is the current pilot proposal. | Stable slots, project manifest and blueprint descriptions are established here. |
| BX-D04 | Keep the requested root mandate filename with a numbered navigation stub. | Authoring choice for this pilot. | One canonical mandate body and explicit manifest path. |
| BX-D05 | Keep BoxUI rendering, host interaction and domain execution responsibilities distinct. | Architecture recommendation under the mandate. | Complete allocation/contract study before implementation. |
| BX-D06 | Start with static and in-process interactive slices before optional network transport. | Delivery recommendation. | Test contracts without making TCP a prerequisite. |
| BX-D07 | Add a versioned Analyzer adapter for this profile. | Proposed future integration based on inspected current limits. | No compatibility claim or current Analyzer change. |

For later entries record trigger/source, alternatives, selected meaning, affected
IDs, decision authority, superseded position and verification consequences.
Issue comments can be sources; the current decision and affected records belong
here. Do not silently relabel a recommendation as owner-approved.


## Additive decisions for draft1

BX-D04 is superseded by the owner's relocation request: the canonical mandate
now lives at 01--Mandate/01-01--Mandate.md; old root file/navigation stub removed.

| ID | Choice | Status / alternative / consequence |
|---|---|---|
| BX-D08 | XFMD is the required native interactive host, in parallel with fork work | Owner direction 2026-09-18; supersedes optional-host/T05 wording. No HTML delivery. |
| BX-D09 | Dedicated BoxUI tree and additive API | Design recommendation; Graph/treemap reuse would lose widget/binding semantics. |
| BX-D10 | Strict JSON after version header; compiled widget registry | Review candidate; smaller parser surface than a fresh textual DSL, no document code. |
| BX-D11 | SVG + typed control map + FOX text overlay | Review candidate; SVG-only cannot edit; browser conflicts with host direction. |
| BX-D12 | In-process synthetic participant; no TCP first | Review candidate; exercises contracts without network lifecycle. |
| BX-D13 | Real bounded design-core SDL plus explicit contract artifacts | Review candidate; unsupported behavior remains documented rather than masquerading as compiled SDL. |
| BX-D14 | Stop at G-PARALLEL-REVIEW | Owner direction; recap/design iteration precedes XFMD implementation. |

## Implementation authorization

BX-D15, 2026-09-18: the owner accepts the proposed design, explicitly waives
another design iteration and requests renderer implementation with phase branches
and milestone commits. This supersedes BX-D14's stop gate for implementation.
XFMD is assigned to a separate agent; this phase does not edit XFMD.

Phase 046 uses `phase/boxui-046-implementation` in an isolated worktree because
the original worktree already contained uncommitted phase-045 source. Those
files are preserved, not overwritten or assumed verified. This phase starts
from reviewed handoff `7076cac` and follows BX-HOST/0.1-draft1.

Terminology clarification: `boxui 0.1` is this extension's new source profile,
not a confirmed version of Concept1's React UIBox. Concept1 uses JSX/props/CSS;
shared source definitions require a compatible adapter, not execution of JSX
inside Rust. Existing Concept1 code and layout behavior remain unchanged.
