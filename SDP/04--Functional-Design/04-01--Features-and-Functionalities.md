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


## First allocation

BX-F01 also addresses BX-R13; BX-F02 addresses BX-R13/BX-R15; BX-F04 addresses
BX-R15. Static export under BX-F01 addresses BX-R14. Stable identities below map
to the SDL model and the detailed function table (all planned).

| Functionality | Local Unit | Reused by | Requirements |
|---|---|---|---|
| ParseBoxUi | BoxUiParser | BX-F01 | BX-R01/08/09 |
| AllocateRegions | BoxUiLayout | BX-F01/02 | BX-R02/03/08 |
| DefineWidgets | WidgetRegistry | BX-F01/02 | BX-R04/12 |
| PrepareFrame | BoxUiPainter | BX-F01/02 | BX-R03/07/14 |
| PublishFrame | PresentationCoordinator | BX-F01/02 | BX-R05/07/13 |
| RouteIntent | PresentationCoordinator | BX-F02/03 | BX-R04/06/12/13 |
| RetainDraft | FoxOverlay | BX-F02 | BX-R05/12 |
| SimulateActivity | SyntheticParticipant | BX-F03 | BX-R10 |

Discovery in BX-F04 uses the existing pilot manifest; future Analyzer functionality
is not allocated as a new renderer runtime Unit. Widgets do not call Feature internals.
