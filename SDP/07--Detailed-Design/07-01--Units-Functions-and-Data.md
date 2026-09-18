---
document_id: SDP-07-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Units functions and data

## Design records to complete

Name requirement-relevant Functions for parsing, validating, measuring, laying
out, preparing a scene, accepting input and publishing a frame. Each needs
input/result types, preconditions, allowed effects and outcome-specific guarantees.
Do not mirror every Rust helper in SDL.

Candidate data identities: BoxUI document/profile, region/widget definition and
instance, binding, source revision, viewport revision, prepared frame, input event
and command correlation. Identify which state is transient, retained or external.

## Unresolved semantics

Set rules for duplicate IDs, scoped references, unknown widget kinds, property
types, recursion, invalid sizes, child diagram failure, stale bindings and removed
controls. Commands, Values, Functions and Activity instances remain distinct.
Choose concrete data/interaction contracts before generating executable IR.

Bindings to Rust symbols remain empty until implementation exists. Existing
renderer entry points are study evidence, not a claim they implement BoxUI.

