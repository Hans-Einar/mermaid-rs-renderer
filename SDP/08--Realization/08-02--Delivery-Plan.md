---
document_id: SDP-08-02
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# Parallel delivery and stop gate

## Phase 046 implementation update

The owner accepted draft1 and authorized renderer implementation and milestone
commits (BX-D15), superseding the stop gate below. R1/R2 now have implementation
commits and runnable fixtures. R3/R4 evidence and the exact integration seam are
recorded in [the handoff](08-03--Renderer-API-and-XFMD-Handoff.md) and
[verification evidence](../09--Verification/09-02--Evidence.md). XFMD runs in a
separate workstream. The following gate text preserves the original design
handoff history; it is not a renewed implementation stop instruction.

R1–R3 and the renderer subset of R4 are now delivered on
`phase/boxui-046-implementation`. Joint R4/X4 remains open until XFMD pins the
fork commit and supplies real GUI/FFI/PDF integration evidence.

## Current assignment and milestones

BX-M01: preserve supplied mandate/structure as a traceable baseline (`d1e70c4`).
BX-M02: inspect actual code/SDL, specify host/profile/contracts, allocate roles,
write structural SDL and boundary fixtures, and register the XFMD task.
BX-M03: run authoring checks, review cross-repository consistency and deliver
recap at **G-PARALLEL-REVIEW**. No renderer or XFMD runtime implementation here.

## Gate G-PARALLEL-REVIEW

Ready for owner review means: source grammar/model has examples and negatives;
frame/input identities, ownership, layout/measurement, failures, native editing,
PDF and budgets are written; each side has named work packages and mock boundary
fixtures; unsupported SDL semantics are explicit; all records/links resolve.
It is not design acceptance, runtime verification or a promise to start XFMD.
After recap and an explicit reviewed revision, both streams may start together.

## Separate workstreams after the gate (all planned)

| Package | Renderer fork | XFMD project | Join evidence |
|---|---|---|---|
| R1 / X1 | typed BoxUI parse/validate, source model, built-in registry | Cmark fence recognition, typed model, new parser injection using fake implementation | same source/model fixtures, negative profiles |
| R2 / X2 | measured layout, child adapter, static/preview SVG + control map | placement/SVG and fake frames, FOX overlays, transforms, focus | same frame fixtures, overlay/clip test |
| R3 / X3 | cancellation, limits, diagnostics, registry capabilities | bind state, input ledger, draft retention, synthetic participant | event/result fixtures, stale-frame oracle |
| R4 / X4 | real frame output and all relevant fork regressions | real adapter pin, mixed Markdown/GUI/PDF and installation checks | cross-repo exact commits, static+interactive witness |

X1–X3 do not depend on R2 completion: use the contract and labelled mocks.
R1–R3 do not depend on FOX: use controlled metrics and mock child preparation.
Neither side calls a missing producer “implemented” because the mock passes.
Only the final join updates XFMD's dependency pin to a verified fork commit.
Do not patch `.deps/` or deploy speculative changes to XFMD main.

XFMD task is recorded under `docs/design/boxui-integration.md` and Sprint 003,
phase 044, in its dedicated worktree. It is a separate project using existing
XFMD workflow, not a migration of that repository into this SDP pilot.
Optional network streaming, broader widget sets, SDL interpreter and Analyzer-site
adapter are later increments, not prerequisites or substitutes for interactive XFMD.
