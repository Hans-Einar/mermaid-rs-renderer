---
document_id: SDP-07-02
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Presentation and widgets

## First profile proposal

Authoring uses `boxui 0.1` followed by strict JSON; see
[practical Markdown fixture](../09--Verification/fixtures/activity.md).
Nested row/column regions, text/value, button, single-line string input and
embedded diagram are the complete first profile. Grid, weighted treemap, tabs,
collapse, arbitrary CSS, plugin loading and nested BoxUI are deferred with explicit
unsupported diagnostics. No implied compatibility with all Concept1 UIBox props.

Widget versions are 1. `valueBinding` reads a typed value; `commandBinding` emits
a command argument. Input binds string value and string command; button command
has no argument. Domain bindings are supplied by the host registry; the document
only declares the names/types it expects. Availability comes from host snapshot.
Missing/stale values have textual status, not color alone. Text/labels are literal,
XML-escaped, and cannot inject Markdown/HTML/SVG.

## Deterministic layout algorithm proposal

Root fills the host-provided block viewport. Initial height is 480 logical px;
width follows available Markdown width. Spacing/padding are theme tokens (8 px),
not CSS. Child order is source order and also defines tab order. Intrinsic text
uses the host font set and actual measurement. Labels wrap at word boundaries;
long unbroken words clip with an overflow diagnostic and full tooltip text.

Measure bottom-up: text min is longest unbroken word, preferred is unwrapped;
button/input min width 96, height 32; value min width 96, height 32; diagram min
160 along main axis. Region min is sum of children minima + gaps along main axis,
maximum of cross minima + padding across it. Explicit size min/max constrain
main axis only; defaults are intrinsic minimum, unbounded max and grow=0 for
controls/text, grow=1 for diagram/regions. `max < min` rejects. No percentages.

Allocate top-down: reserve minima and spacing; distribute positive remainder by
nonzero grow, clamping maxima iteratively with source-order ties. If all weights
are zero leave trailing space. Cross axis stretches inside region padding.
Re-measure wrapped text at allocated width, then propagate required height once;
if unresolved minimum overflow remains, return `layout-no-space` with affected IDs.
No unbounded reflow loop, hidden shrunken labels or negative dimensions. No automatic
responsive reordering; author can choose column. Host outer document scrolls.
Inside a diagram pane use contain scaling, centered, clipped, no child interaction.

Existing Mermaid content initially supports flowchart, sequenceDiagram and
stateDiagram-v2 only through XFMD's existing bounded profiles. Requesting another
family produces a local unsupported-pane diagnostic, not reinterpretation. Extend
child capability table incrementally. For semantic consistency XFMD interprets children in its interpreter and prepares their typed models
in its application chain before parent layout; fork standalone adapter declares its
own supported subset. Parent host contract includes capability IDs. Children never
read files, fetch URLs or recursively invoke BoxUI. Failure paints an in-pane
message and preserves siblings; failure of parent layout preserves old full frame.

## Appearance and interaction

Library renders static controls, disabled/pending state and focus decoration from
explicit presentation state. Native text overlays supply actual editing in XFMD.
Palette and font signature are explicit preparation inputs. Light/Dark and reading
sliders update both graphics and overlays coherently; text-field background matches
its surface. Use host-established fonts and avoid relying on platform default sizes.
PDF paints full static fields from accepted values, excluding drafts.

[Host contract](../06--Container-Design/06-02--XFMD-Host-Contract.md) defines focus,
clipboard, input transform, compatible replacement, editing conflicts and IME limits.
Layout weights do not bind to changing measurements. Future ports can use these
contracts without importing FOX into the library.
