---
document_id: SDP-04-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Features and functionalities

## Candidate feature model

| ID | Feature | Local responsibilities to allocate | Requirements |
|---|---|---|---|
| BX-F01 | Describe and view a composed UI | Parse and validate BoxUI; measure/layout regions; render widget and diagram content | BX-R01, BX-R02, BX-R03, BX-R08, BX-R09 |
| BX-F02 | Interact with a modeled system | Resolve bindings; route input; preserve editing context; expose outcomes | BX-R04, BX-R05, BX-R06, BX-R07, BX-R12 |
| BX-F03 | Explore partial implementations with traceable evidence | Run scenario stimuli; identify simulated participants; check outcomes | BX-R10 |
| BX-F04 | Navigate design and delivery from SDP root | Discover records; resolve relations; list targets and views | BX-R11 |

Feature responsibilities may cross runtime boundaries. Each allocated
Functionality must have one accountable local Unit; early allocation can remain
unresolved. Do not turn this list into final Containers or one function per
responsibility before architecture/design analysis.

BoxUI is a reusable presentation capability. The prototype's domain simulation
is a collaborator, not a Feature that must be hardcoded into the renderer.

