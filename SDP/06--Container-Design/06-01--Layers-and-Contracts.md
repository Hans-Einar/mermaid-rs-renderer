---
document_id: SDP-06-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Internal layers and contracts

## Applicability

This standard slot includes internal library design when there is no standalone
Container. Do not invent a service to satisfy the folder name.

Candidate internal responsibilities are source parsing, semantic validation,
layout/measurement, widget/diagram scene production and host interaction adapters.
Assess their mapping to existing modules before adding directories or public APIs.

## Contract checklist

For each boundary define typed input/output, ownership/lifetime, source identity,
revision handling, failure/diagnostic behavior and resource limits. Keep generic
widget definitions independent of Ponsse domain concepts and native host widgets.

A host adapter must preserve Representation/Composition/Presentation/Renderer
responsibilities. A layout result cannot take domain authority. Embedded diagrams
have explicit content scope and bounded rendering; errors must be attributable
to the relevant node and cannot corrupt sibling state.

The detailed design must decide whether interaction information augments an
existing scene or needs a separate typed result with common revision identity.
This document does not choose an ABI.

