---
document_id: SDP-01-01
profile: sdp-system-development-pilot/0.1
status: authorized-direction
updated: 2026-09-18
---

# Mermaid BoxUI extension mandate

## 1. Mandate and authority

Develop **BoxUI** as a separately identifiable extension of the project's
`mermaid-rs-renderer` fork. The owner explicitly permits extending our Mermaid
implementation and choosing the name **boxui** rather than changing treemap.
Add a reusable widget library and the contracts needed to compose diagrams and
interactive controls in nested UI regions.

Use this project to pilot the numbered **System Development Process (SDP)**
structure. **SDL** is the System Description Language used to describe the design;
the project must expose explicit discovery points for future parser/compiler/
interpreter tooling and SDP-Analyzer.

This mandate records the owner's direction. Detailed syntax, module allocation,
widget APIs and protocol choices below remain design work unless explicitly
identified as constraints. Creating this mandate does not claim implementation,
global adoption of the pilot structure or approval of a final UI design.

## 2. Problem and intended value

We can render static diagrams but cannot yet describe and exercise a UI made of
nested boxes, live Values, Commands, editable controls and embedded diagrams
through a common design model. Layout proposals and behavioral scenarios therefore
remain disconnected from the actual interaction they are intended to explain.

BoxUI should let designers describe a proposed interface, render it, interact
with a bounded simulated system, and trace observed behavior back to requirements,
Features, Functionalities, Activities and state. Missing implementation can be
represented by explicit simulated participants; it must never be disguised as
verified production behavior.

## 3. Required outcomes

- A named, versioned BoxUI syntax/profile and semantic model with stable node IDs.
- A compositional layout and rendering path for nested regions and registered
  widgets; supported Mermaid diagram types can be embedded as diagram content.
- A widget contract covering appearance, values, interactions, focus and lifecycle.
- Static output usable in documentation and a host-facing interaction contract
  usable by XFMD or another renderer host.
- Explicit Value/Representation and Command bindings suitable for future SDL
  interpreter use, without placing domain logic in the graphics library.
- A discoverable SDP project with standardized numbered entry documents,
  module/target inventory, blueprint descriptions and traceable evidence.
- A bounded vertical demonstration with declared simulated behavior, followed by
  evidence of which requirements were actually checked.

## 4. Scope

### BoxUI language and model

Use a distinct `boxui` entry identity and explicit profile version; settle the
exact grammar during design. Support readable authoring, useful diagnostics,
source locations and stable identity across layout changes. Reject unknown or
unsupported semantics rather than silently converting them to decorative text.

Model nested layout regions, widget instances and diagram content as distinct
roles. Define sizing, measurement, overflow, clipping, ordering and viewport
response. Study reuse of existing BoxUI/BOX contracts and renderer abstractions
before inventing parallel mechanisms. Treemap's weighted-area algorithm may be
an optional policy; it must not dictate every UI layout.

### Widget library

The first useful set is a text/value display, button, text input and diagram
pane inside row/column regions. Define a registration/extension mechanism so
additional widgets do not require a separate parser dialect per widget.

Each definition declares kind/version, properties and value types, events,
validation, sizing, accessibility semantics and relevant focus/disabled/error
states. Distinguish definition, instance, runtime interaction state and externally
owned domain state. Decide how an input draft becomes a committed typed intent.

### Embedded diagrams

Compose separately parsed/laid-out diagram content within allocated BoxUI
regions, with predictable scaling and clipping. Preserve diagram identity and
diagnostics. Declare supported child profiles and resource limits. Nested BoxUI
may be studied later; recursion, name collisions and unbounded expansion must
not be accidental consequences of embedding.

### Renderer and interaction boundary

Expose typed input events and geometry/binding information sufficient for hit
testing, keyboard focus and editing. SVG alone is not an interactive text-control
implementation. Determine which responsibilities belong to the library and which
to the host; demonstrate that boundary with a real host adapter or minimal host.

Keep visuals, input regions and bindings at the same frame revision. Preserve
stable controls, focus and unsubmitted input where the contract says they survive.
An old-frame event must not accidentally invoke a new widget's binding.

### SDL execution and stream integration

Define integration contracts for a sequencer/SDL execution slice and an optional
XFMD renderer connection. Scenario scripts supply inputs or explicitly simulated
participants; assertions check obligations independently of scripted output.

Study staged content, render readiness and frame publication. A stream protocol
must distinguish staging clear from active display, rendering from presentation,
and input identity from a current screen coordinate. TCP is a candidate transport,
not an already selected wire protocol. Do not start a network service merely by
loading or rendering a BoxUI document.

### SDP structure experiment

Populate the standard entry documents enough to reveal current decisions, gaps
and next work. Enumerate source sets, modules, run/test/generation targets and
blueprint descriptions in the manifest. List blocked capabilities truthfully.
Keep numbering aligned with design questions while allowing iterative work.

## 5. Boundaries and preserved obligations

- Preserve current Mermaid diagram behavior; do not redefine `treemap-beta`.
- Keep parse/model, layout, rendering and host interaction responsibilities
  explicit; assess existing interfaces before changing them.
- Preserve the selected Representation, Composition, Presentation and Renderer
  separation when binding SDL/MVP1 UI concepts. BoxUI is a presentation mechanism,
  not a new owner of machine or bucking domain state.
- Existing standard diagrams remain usable without BoxUI execution dependencies.
- Keep offline deterministic fixtures and bounded rendering. Dynamic values must
  not change layout weights unless explicitly bound to that property.
- Host/toolkit specifics and Ponsse-specific policy stay outside generic widgets.
- A simulated participant, missing binding and native implementation have different
  evidence status. Visual plausibility is not acceptance evidence by itself.
- Changes in XFMD, SDP-Analyzer and shared SDP tooling are separately identified
  integration work. This branch owns the renderer extension and its contracts;
  cross-repository delivery must record its own scope and evidence.

## 6. Non-goals for the first increment

No full application framework, complete HTML/CSS engine, arbitrary embedded code,
universal production UI toolkit, complete SDL runtime, or automatic execution
of Mermaid sequence diagrams. No migration of all existing SDP projects and no
claim that every Mermaid construct is supported inside BoxUI immediately.
No physical machine, serial transmission or actuator integration is part of the
prototype. It uses synthetic/offline input.

## 7. First vertical experiment and acceptance direction

Render a synthetic Activity panel containing a current measurement, work-context
input, Suspend/Resume buttons and an embedded state or sequence diagram.
A declared simulated participant supplies observations and Activity outcomes.

Demonstrate a layout change while an input is being edited, suspension and
resumption under valid context, rejection under changed context, and a failed
render that preserves the active frame. Record model/source revisions, event
identities, observed outcomes and which participants were simulated.

Acceptance must cover:
1. Valid and invalid BoxUI syntax with precise diagnostics.
2. Stable IDs, well-defined ownership and typed widget bindings.
3. Nested layout, long labels, clipping and selected viewport sizes.
4. Embedded diagram success/failure without corrupting sibling controls.
5. Mouse and keyboard input, focus and draft preservation.
6. Event/frame correlation, stale result handling and no duplicate committed intent.
7. Static SVG output and a declared interactive-host witness.
8. Existing Mermaid regression checks appropriate to changed code.
9. Discoverable project records and blueprint descriptions with no invented
   runnable targets or successful evidence.

Detailed acceptance cases and measured thresholds belong in
[requirements](../03--Requirements/03-02--Requirements.md) and
[verification](../09--Verification/09-01--Verification-Plan.md).

## 8. Delivery and completion

Proceed through the [delivery plan](../08--Realization/08-02--Delivery-Plan.md):
study and contracts; static BoxUI slice; interaction slice; scenario/host integration.
A Tier is a vertical usable increment, not a horizontal layer or a renamed folder.

At each step update the relevant SDP design, decisions, bindings and evidence.
Do not wait until implementation is complete to document constraints discovered
during it. Reopen earlier design explicitly when needed.

The extension is complete only for its declared version/profile when its contract,
implementation, regression evidence, authoring examples and host boundaries agree.
A standard SDP release and a complete SDL interpreter remain separate outcomes.

## 9. Open design decisions

Exact grammar/version marker; reuse of existing BoxUI contracts; layout constraint
model; static versus interactive widget backend; widget extension ABI; input and
focus ownership; text editing/IME requirements; nested diagram recursion; protocol
framing and recovery; scenario-file semantics; evidence and model schema promotion.

Resolve these through the numbered documents and decision log. Do not treat a
convenient first implementation as an unstated architecture decision.

## 10. Reading order and source basis

Read [the project index](../00--Project/00-01--Project-Index.md),
[study](../02--Study/02-01--Study.md), functional design, architecture and detailed
design before implementing. The study distinguishes current checkpoint concepts,
older SDL research, inspected renderer code and future Analyzer capabilities.

