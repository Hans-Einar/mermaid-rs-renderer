---
document_id: SDP-11-02
profile: sdp-system-development-pilot/0.1
status: pilot
updated: 2026-09-18
---

# Traceability and change impact

## Record roles

The manifest's relations link registered documents, modules, targets and
blueprints. Requirement, Feature, Use Case and Activity IDs currently appear in
their canonical numbered documents. Their structured semantic registry is not
yet implemented; a future adapter must report that distinction.

A requirement-to-document edge locates explanation, not proof of satisfaction.
An execution target with a command is not passed evidence.
Later model-derived relations should replace manual duplicates with stable
source references and schema-versioned records.

## Change impact

Trace need -> requirement -> functional responsibility/Activity -> allocation
and contract -> Function/widget binding -> source/IR -> check/evidence.
Trace backward when implementation reveals a missing condition or broken rule.

Include cross-cutting ownership, identity, viewport, lifecycle and compatibility
obligations. Keep accepted, proposed, implemented, observed and unknown facts
distinct. Do not use complete file counts as design-completion evidence.


## Draft1 tracing and change rules

Use Cases/requirements -> feature table -> named SDL Functionality -> detailed
function table -> contract section -> BX-ATxx. The structural SDL source preserves
ownership/capability facts only; requirement/temporal references remain in these
records. Binding/type change affects both parser and host; geometry change affects
layout, hit tests, overlay and PDF; session identity change affects ledger, drafts,
worker cancellation and synthetic oracle. Update both tasks before parallel edits.
All product source bindings are still Planned. No trace link means proof of test.
