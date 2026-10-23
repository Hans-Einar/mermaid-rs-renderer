---
document_id: SDP-00-02
profile: sdp-system-development-pilot/0.1
status: pilot
updated: 2026-09-18
---

# Numbered SDP structure profile

## Purpose

Profile ID: `sdp-system-development-pilot`, version `0.1`.
This is the first project experiment for a reusable numbered structure, not a
released replacement for Standard Document Procedure or its installer.

Numbers identify engineering questions in a progression. They are not workflow
locks, Sprint numbers, runtime layers, maturity scores or approval by themselves.
Work may move upward when a detailed finding changes an earlier assumption.

## Standard directory and document slots

| Folder | Standard documents | Primary question |
|---|---|---|
| 00--Project | 00-01 Project Index; 00-02 SDP Structure; 00-03 Discovery and Tooling Contract | Where is the current project model, and how is it discovered? |
| 01--Mandate | 01-01 Mandate | Why does this project exist, with which scope and outcomes? |
| 02--Study | 02-01 Study | What evidence, alternatives and unknowns shape the solution? |
| 03--Requirements | 03-01 User Needs and Use Cases; 03-02 Requirements | What must users achieve and what must hold? |
| 04--Functional-Design | 04-01 Features and Functionalities; 04-02 Activities and State | Which responsibilities and behavior deliver the outcomes? |
| 05--System-Architecture | 05-01 Containers and Channels | Which boundaries collaborate and own responsibility? |
| 06--Container-Design | 06-01 Layers and Contracts | How is each runtime boundary or library structured internally? |
| 07--Detailed-Design | 07-01 Units Functions and Data; 07-02 Presentation and Widgets | What precise operations, data and lifecycle contracts realize the design? |
| 08--Realization | 08-01 Modules and Bindings; 08-02 Delivery Plan | What code, IR and adapters exist, and what bounded increments are next? |
| 09--Verification | 09-01 Verification Plan; 09-02 Evidence | What will check obligations, and what actually passed at which revision? |
| 10--Blueprints | 10-01 Blueprint Catalog; numbered blueprint descriptions | Which named views can be generated, from which facts and rules? |
| 11--Decisions-and-Traceability | 11-01 Decision Log; 11-02 Traceability | Why is this the design, and how do intent, realization and evidence connect? |
| 12--Delivery | 12-01 Delivery and Compatibility | What can be delivered and consumed, with which compatibility limits? |

The last three areas support all stages. A blueprint can describe requirements,
architecture, behavior or UI: folder 10 does not mean blueprints begin after coding.

## Naming and stable identity

Use `AA--Area/AA-BB--Document-Name.md`. The stable slot ID is `SDP-AA-BB`;
cross-project identity is `projectId + documentId`. Documents may have children
or referenced records without changing their slot. Do not renumber an established
identity merely to reorder navigation. Add or revise profile slots explicitly.

The canonical mandate is `01--Mandate/01-01--Mandate.md`.
The manifest maps `SDP-01-01` to this standard numbered location;
there is no separate navigation stub or second mandate body.

Each canonical Markdown document begins with `document_id`, `profile`,
`status` and `updated` metadata. Status values for this pilot are
`authorized-direction`, `pilot`, `draft`, `inventory`, `planned` and
`evidence`; none automatically means implementation verified.

All standard slots exist in a new pilot project. An inapplicable slot records its
reason instead of inventing a Container or irrelevant design. Empty/TBD-only files
do not meet a stage's completion criteria. Project-specific details link from
these stable entry documents. A later standard may refine this rule based on use.

## SDL placement and authority

An abstraction-level document can reference an SDL source set stored beside its
design records. Folder placement alone cannot declare a System, own a Unit,
change a keyword's meaning or make source files part of a compilation.
Explicit manifest source sets list the language profile, member files and entry
points. Exactly one System per selected system entry point follows the current
language direction; independent models need distinct entry identities.

Markdown explains decisions and unresolved details. Once a fact has canonical
SDL representation, documents and blueprints reference it instead of keeping an
independent contradictory copy. Generated outputs go under the manifest's
generated root and never become authoring inputs by directory scanning.

This pilot intentionally contains no invented executable `.design` files.
Current SDL grammar does not yet express the full proposed BoxUI system.

## Profile evolution and old-to-new map

Old 01/02/03 retain their intent. Old 04 Architecture splits into functional,
system and container design; old 05 DesignAnalysis feeds those studies and the
delivery plan. Old 06 Design maps to 07; old 07 Implementation maps to 08.
Verification, Traceability and release concerns gain explicit numbered slots.

This is a conceptual migration map, not a script to rename existing projects.
Version changes record added/retired slots, semantic changes, aliases, tool
compatibility and migration instructions. Preserve old evidence and stable IDs.
Promote a profile to a shared SDP release only after testing it on this project
and at least one materially different project.

