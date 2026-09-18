# BoxUI extension — SDP project entry

This is the first **System Development Process** numbered-structure pilot.
The mandate authorizes the BoxUI direction; syntax, detailed architecture and
runtime integration still require design and implementation.

Start with the [mandate](01--Mandate/01-01--Mandate.md),
[project index](00--Project/00-01--Project-Index.md),
[structure profile](00--Project/00-02--SDP-Structure.md) and
[discovery contract](00--Project/00-03--Discovery-and-Tooling-Contract.md).

The machine entry point is [sdp-project.json](sdp-project.json), validated against
[the pilot schema](00--Project/sdp-project.schema.json). It lists existing and
planned modules, bound and blocked targets, and explicit blueprint descriptions.
No current SDL compiler or SDP-Analyzer compatibility is implied.

## Current review entry

[Delivery/recap](12--Delivery/12-01--Delivery-and-Compatibility.md),
[shared XFMD contract](06--Container-Design/06-02--XFMD-Host-Contract.md),
[parallel packages and stop gate](08--Realization/08-02--Delivery-Plan.md),
[SDL structure](07--Detailed-Design/SDL/BoxUi.design),
[practical Markdown fixture](09--Verification/fixtures/activity.md).

XFMD is the required native interactive host. No HTML page is being built.
G-PARALLEL-REVIEW is a review checkpoint, not authorization to start implementation.

## Numbered document index

| ID | Document | Status |
|---|---|---|
| SDP-00-01 | [Project index](00--Project/00-01--Project-Index.md) | pilot |
| SDP-00-02 | [Numbered SDP structure profile](00--Project/00-02--SDP-Structure.md) | pilot |
| SDP-00-03 | [Discovery and tooling contract](00--Project/00-03--Discovery-and-Tooling-Contract.md) | pilot |
| SDP-01-01 | [Mermaid BoxUI extension mandate](01--Mandate/01-01--Mandate.md) | authorized-direction |
| SDP-02-01 | [Study and evidence baseline](02--Study/02-01--Study.md) | draft |
| SDP-02-02 | [Workspace provenance](02--Study/02-02--Workspace-Provenance.md) | inventory |
| SDP-03-01 | [User needs and use cases](03--Requirements/03-01--User-Needs-and-Use-Cases.md) | draft |
| SDP-03-02 | [Requirements](03--Requirements/03-02--Requirements.md) | draft |
| SDP-04-01 | [Features and functionalities](04--Functional-Design/04-01--Features-and-Functionalities.md) | draft |
| SDP-04-02 | [Activities and state](04--Functional-Design/04-02--Activities-and-State.md) | draft |
| SDP-05-01 | [System boundaries and channels](05--System-Architecture/05-01--Containers-and-Channels.md) | draft |
| SDP-06-01 | [Internal layers and contracts](06--Container-Design/06-01--Layers-and-Contracts.md) | draft |
| SDP-07-01 | [Units functions and data](07--Detailed-Design/07-01--Units-Functions-and-Data.md) | draft |
| SDP-07-02 | [Presentation and widgets](07--Detailed-Design/07-02--Presentation-and-Widgets.md) | draft |
| SDP-08-01 | [Modules and bindings](08--Realization/08-01--Modules-and-Bindings.md) | inventory |
| SDP-08-02 | [Vertical delivery plan](08--Realization/08-02--Delivery-Plan.md) | planned |
| SDP-09-01 | [Verification plan](09--Verification/09-01--Verification-Plan.md) | planned |
| SDP-09-02 | [Verification evidence](09--Verification/09-02--Evidence.md) | evidence |
| SDP-10-01 | [Blueprint catalog](10--Blueprints/10-01--Blueprint-Catalog.md) | planned |
| SDP-10-02 | [System responsibilities blueprint](10--Blueprints/10-02--System-Responsibilities.md) | planned |
| SDP-10-03 | [Widget layout blueprint](10--Blueprints/10-03--Widget-Layout.md) | planned |
| SDP-10-04 | [Frame and input interaction blueprint](10--Blueprints/10-04--Frame-Interaction.md) | planned |
| SDP-11-01 | [Decision log](11--Decisions-and-Traceability/11-01--Decision-Log.md) | pilot |
| SDP-11-02 | [Traceability and change impact](11--Decisions-and-Traceability/11-02--Traceability.md) | pilot |
| SDP-06-02 | [XFMD host contract](06--Container-Design/06-02--XFMD-Host-Contract.md) | draft |
| SDP-09-03 | [Acceptance catalog](09--Verification/09-03--Acceptance-Catalog.md) | draft |
| SDP-12-01 | [Delivery and compatibility](12--Delivery/12-01--Delivery-and-Compatibility.md) | planned |

## Current scope

The draft1 design handoff, structural SDL and contract fixtures are delivered here. BoxUI and its widget
library are not implemented. The existing renderer library has a declared Cargo
test target; it was not run for this authoring task. All new compile/run/generation
targets disclose missing bindings. See [evidence](09--Verification/09-02--Evidence.md).

The prior workspace note and its baseline references are preserved through
[workspace provenance](02--Study/02-02--Workspace-Provenance.md) and Git history.
Project-specific facts must not become defaults in future shared SDP templates.
