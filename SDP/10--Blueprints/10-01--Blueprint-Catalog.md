---
document_id: SDP-10-01
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# Blueprint catalog

## Registered views

| ID | Description | Structured descriptor | Readiness |
|---|---|---|---|
| BX-BP01 | [System responsibilities](10-02--System-Responsibilities.md) | [Descriptor](10-02--System-Responsibilities.blueprint.json) | Blocked: formal source and generator not bound. |
| BX-BP02 | [Widget layout](10-03--Widget-Layout.md) | [Descriptor](10-03--Widget-Layout.blueprint.json) | Blocked: BoxUI model/profile and generator not bound. |
| BX-BP03 | [Frame and input interaction](10-04--Frame-Interaction.md) | [Descriptor](10-04--Frame-Interaction.blueprint.json) | Blocked: behavioral model and generator not bound. |

A blueprint is a named projection of model facts with a purpose and preservation
rules. It is not merely “render whichever Mermaid file is found.”
Descriptors select explicit source records, abstraction level, output family,
generator target and output path. Generated artifacts must retain source/model
identities, revisions and gaps. Hand-authored views are not claimed as generated.

A future tool can list these descriptors now, including blockers; it cannot
generate the intended views until the relevant bindings exist.

