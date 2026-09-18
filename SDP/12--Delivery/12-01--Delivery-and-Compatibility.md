---
document_id: SDP-12-01
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# Delivery and compatibility

## Current deliverable

This delivery is the mandate and pilot document/catalog structure.
No BoxUI language/profile release, ABI, package, host integration or shared SDP
standard is released by creating these documents.

## Future release record

Record BoxUI profile version, renderer API and widget-definition compatibility,
host/interaction protocol versions, dependencies, supported feature flags,
supported child diagram profiles, migration instructions and exact evidence.
Keep library, language, widget and SDP profile versions separate.

Existing Mermaid documents must retain their established semantics. A new widget
or layout policy requires an explicit compatibility assessment. Support matrices
must distinguish implemented syntax from future ambitions.

Publishing a shared SDP profile requires reusable templates, tooling support and
a migration contract. Do not copy this project's BoxUI-specific content into
every consuming project; standardize document slots/schema, not project facts.

