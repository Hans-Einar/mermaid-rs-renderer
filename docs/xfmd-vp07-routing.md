# XFMD VP07 routing reproduction and bounded correction candidate

Owner authorizes XFMD Session0005 diagram investigation. Isolated worktree starts
at XFMD's pinned commit 589517a7107ab95ee7dfa5ae88c2f339c0a284dc; no shared fork
branch or XFMD .deps bytes are edited. Initial artifact is a reproduction test,
not an adopted fix or a published release.

XFMD native parser accepts 12 nodes/18 edges for ConfigurableScrolling. Its
measured native layout fails with a two-point diagonal at edge11. Preserve source,
labels, shapes and all relations. Exact XFMD native geometry/error and PDF are
recorded in external N3a-001. Captured Pango label measurements feed the library
reproducer, using XFMD's 0.75 point/pixel conversion and 16px DejaVu Sans theme.

Libavoid connector.cpp explicitly constructs a two-point path and marks reroute
when no path is found. The bridge currently reports the diagonal as generic
Backend(non-orthogonal), while flowchart compute only retries NoSpace. Semantic
layout already retries this error category. Investigate classifying only a
non-orthogonal two-point failure as NoSpace: fixed-position callers still fail,
while fresh-placement callers can use the existing single spacing retry with the
same Libavoid engine and deadline. Keep multi-segment corruption as Backend,
retain complete geometric/obstacle checks and never accept the diagonal itself.

Required evidence before adoption: reproduce the old failure, pass all18 edges
with valid routes on retry, keep fixed geometry/rejection/cancellation tests,
inspect actual XFMD preview/PDF and measure within its production budget. This
candidate does not address the independent nested-group containment problem.
Publishing, dependency pin update and consumer release remain distinct actions.

## Verified candidate — 2026-10-08

Two-point diagonal failure is classified as NoSpace; longer malformed paths remain
Backend failures. The existing one-shot placement retry succeeds with all12 nodes
and18 edges; no Legacy fallback or relaxed geometry validation. Captured exact
Pango metrics reproduce the owner's edge11/coordinates before retry. Twelve
existing libavoid integration tests plus the new VP07 regression pass.

An isolated XFMD C++ consumer linked to a debug FFI build of this candidate renders
one scene and exports one PDF page within the unchanged production2-second layout
budget. Preview and PDF were inspected; all labels and relationships remain.
One optional label leader is omitted with the existing diagnostic; no relation
is dropped. This is not general readability/optimal-layout acceptance.

The temporary consumer workspace uses its recorded candidate Cargo.lock; native
source pin and installed XFMD remain unchanged. Dependency adoption/release and
independent review are still pending. No public push, tag or release performed.
External evidence: XFMD SDP/05--Implementation/SDL-Navigation/evidence/N3a-001.
