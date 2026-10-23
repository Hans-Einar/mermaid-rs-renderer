---
document_id: SDP-06-02
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# XFMD host contract — BX-HOST/0.1-draft1

## 1. Status and compatibility

The owner accepted this design and authorized implementation in BX-D15. This is
still a draft contract, not a released C ABI. Both repositories implement this
exact revision. Source schema, frame schema and fixtures in
[contracts](contracts/README.md) are the shared artifact. Breaking
changes before/after parallel start require an explicit revision and both tasks
updated; never silently widen accepted input. The renderer's concrete Rust API
and implemented subset are recorded in the
[phase-046 handoff](../08--Realization/08-03--Renderer-API-and-XFMD-Handoff.md).

## 2. Source and model

Canonical Markdown fence is `boxui`, body begins `boxui 0.1` then one strict JSON
object. Also accept a `mermaid` fence whose first nonblank line is exactly
`boxui 0.1`; both invoke the same BoxUI interpreter, not the Mermaid Graph parser.
Plain code and `.txt` remain literal. Header version must match schema profile.
JSON is a deliberately bounded authoring representation, not an adopted SDL DSL.
Reject duplicate keys, comments, trailing commas/bytes, nonfinite numbers,
unknown fields and unknown widget versions. Decode UTF-8 strictly. Diagnostics
carry code, severity, node ID if known and byte-offset half-open source range;
XFMD maps the range through the fence SourceRange, explicitly approximate if needed.

`parse_boxui(source, limits)` -> validated typed document/diagnostics. A separate
`prepare_boxui(document, snapshot, viewport, palette, metrics, budget)` never
re-parses source. A portable model serialization uses boxui-model.schema.json; child source is
replaced by childRef, with default sizing resolved by the layout policy; C++ owns FOX/Rust-free values. Re-serialization must
preserve binding declarations, children order and IDs, not raw JSON formatting.

## 3. Identity and snapshots

IDs in source match `[A-Za-z][A-Za-z0-9_-]{0,63}`. documentId is unique among
BoxUI blocks in a Markdown document; node IDs are unique inside it. Reject all
conflicting duplicate blocks, not first-wins. Runtime identity is
(window session nonce, document epoch, documentId, nodeId). File switch/reload
creates a new epoch and clears runtime bindings; unsaved edits preserve epoch.
Delete+reinsert increments a host incarnation counter. A node at the same index
is never sufficient identity. Undo of a deleted block cannot resurrect old intents.

FrameKey = session, epoch, blockId, incarnation, sourceRevision,
bindingRevision, stateRevision, viewportRevision, themeRevision, frameSequence.
All counters are monotonic unsigned 64-bit values serialized as decimal strings;
wrap requires a new session. A frame key is compared by equality, not a timestamp.
StateSnapshot carries a complete typed value map and command availability map;
values distinguish missing/current/stale, never infer currentness from presence.
No source-provided binding grants permission. Host registration resolves names,
context and types, and gives provenance native/simulated/unbound. Unbound read
values show “unbound”; actions are disabled with a visible reason. A document
cannot name a shell command, URL or file to execute.

## 4. Prepared frame and publication

Frame contains key, complete `staticSvg`, `previewSvg`, width/height, diagnostics,
ordered controls, and text-edit surfaces. SVG is inert and self-contained: no
scripts, external resources or foreignObject. Embedded child IDs are namespaced.
Static SVG includes text-field text; preview SVG leaves native edit interiors
unpainted. Both are produced together from identical geometry. The control map
includes node/kind, rect+clip, enabled, role, accessibleName, commandBinding, optional valueBinding and type.
Only visible/intersecting controls enter focus order; visual clipping limits input.

GUI publication is one guarded operation: verify current request key, build hidden
native edit controls, reconcile retained state, then swap visuals/control map/
overlays together on the GUI thread. Failure leaves the previous full frame.
Do not acknowledge FramePresented until the accepted frame is installed; this
is not a physical display scan-out guarantee. A malformed replacement keeps the
old frame visibly labelled stale and disables new command submissions until a
valid frame publishes. Old replies can still complete their original ledger entry.
On initial failure show source and diagnostic; surrounding Markdown stays usable.

Source/binding/context changes invalidate command dispatch immediately, even
while old visuals remain. While only geometry/theme preparation is pending, input
uses the visible frame if its source/binding/context is still current. A newly
published frame invalidates held pointer capture; release never targets new geometry.
Late worker results are discarded without callbacks into removed documents.

## 5. Coordinates, keyboard and text editing

All frame rectangles are finite SVG logical px, origin top-left, axis-aligned.
XFMD maps px -> renderer points with 0.75, then document placement, preview scale
and scroll offset. Input applies the exact inverse; overlays apply the same
forward transform. Intersect block, pane and viewport clips. Test 100%/150%/200%,
Wrap/A4, document scroll and resize. Partly clipped input surfaces are hosted
inside a clipped native child area, not allowed to cover adjacent Markdown.

Buttons use SVG plus semantic pointer/keyboard dispatch; inputs use native
FXTextField hosted by a separate FoxBoxUiOverlay. Do not implement caret/IME by
painting Pango text. Tab/Shift-Tab follow document order through enabled widgets
then back to normal XFMD controls. Button activates once on matching press/release
or Enter/Space key release; suppress repeats. Input Enter commits, Escape restores
latest accepted value, Tab leaves the draft unsubmitted. Ctrl+C/V/A belong to a
focused field; outside widgets ordinary Markdown selection and links behave as now.
Global save remains Markdown save; it never implicitly commits widget edits.

Native text selection/caret/draft survive only when session+epoch+block+incarnation+
node+kind/version+value type+binding ID/revision/context match. Layout/theme change
alone does not discard them. Incompatible replacement cancels draft, removes focus
and shows a notice; do not silently submit. Externally updated value while a dirty
draft exists becomes the latest accepted value but leaves draft intact and shows
conflict. Enter carries expected value revision; reject stale rather than overwrite.
Basic Unicode keyboard/edit/clipboard is required. Native IME composition must be
tested on target desktop; if composition cannot safely migrate, defer compatible
publication until composition ends. Do not claim full screen-reader/IME parity.
Role/name/state descriptors are mandatory; AT-SPI integration is a later capability.

## 6. Intent and result

InputIntent fields: key, widgetId, bindingId, contextRevision, eventId,
commandId, expectedValueRevision (input only), action activate/commit, typed value
(input only). Host allocates eventId; commandId is stable for one submission.
Revalidate enabled state, type, context and binding at dispatch, independently
of SVG appearance. Renderer does not execute commands. At most one pending command
per control; repeated Enter/click while pending reports busy, not a new command.

Results: accepted/rejected/pending/unknown, same commandId and context, optional
validation message. Accepted means participant accepted the command, not that
all domain postconditions passed. Observation updates, not optimistic repaint,
change accepted Values. Timeout becomes unknown; no automatic retry. Session-local
ledger retains completed IDs for session lifetime within the command limit;
duplicate matching IDs return the stored result; conflicting reuse rejects.
No exactly-once guarantee across crashes or remote transport. Closing a session
cancels pending local delivery and releases resources; no replay on restart.

## 7. C++/Rust bridge (planned, XFMD-owned)

Add independent `xfmd_boxui_parse_v1`, `xfmd_boxui_prepare_v1`,
`xfmd_boxui_result_free_v1` in XFMD bridge/BoxUiAbi.h, not the generic crate.
Signature pattern mirrors XfmdDiagramResult: ABI/struct size/status, owned bytes,
size and owner; measured callback is borrowed synchronously. Input/output bytes
are bounded UTF-8 JSON envelopes tagged `BX-HOST/0.1-draft1`; separate entry points
keep source parsing out of renderer. Existing diagram ABI is unchanged.
Free exactly once even on error; free zeroes result. Borrowed bytes end at call
return, result data lives until free. No Rust object layout exposed to C++.

Cancellation callback is cooperative and polled between parse/layout/child stages;
no hard wall-clock guarantee from an in-process C ABI. Callback context lives for
call duration only. A host timeout discards an eventual result; never kill an
in-process thread or free its context while it is still executing.

## 8. PDF and limits

Export captures an immutable per-document snapshot on the GUI thread, then uses
static SVG on the export worker with print palette and the same measured geometry.
Include accepted values/status and a “simulated snapshot” marker when relevant;
unsubmitted drafts are excluded. No live control overlays or event dispatch in PDF.
Prepared preview is not a current export snapshot by assumption. Page fit preserves
aspect and clips no widget labels silently; oversize errors are explicit.

Limits per block: 256 KiB UTF-8 source, 256 nodes, depth 16, 8 child diagrams,
64 KiB text per child, 4096 Unicode scalars per text input, 8 MiB complete frame
(two SVGs + metadata). Per document 64 combined Mermaid/BoxUI blocks, 64 MiB active
frame resources. At most one running and one replaceable pending preparation per
session; older pending presentation updates may coalesce. Commands never coalesce.
Command queue 64, session ledger 4096; exhaustion visibly refuses new submissions
until explicit session restart, not eviction that enables re-execution.
Layout budget 2000 ms release / 10000 ms sanitizer including children, cooperative.
SVG 8 MiB includes both variants; viewport max 8192 per axis; min usable 320x240.
Exact values are review candidates. Test boundary-1/boundary/boundary+1.

## 9. Payload completion and codec

The prepare/result schemas accompany source/frame/intent. Prepare.model is separately
validated by the normalized model schema; structural validation alone never supplies binding
types. Value validity=missing requires null; otherwise exact declared scalar type.
Snapshot lists must resolve the source binding IDs exactly once; an absent host
binding is represented explicitly by unbound command or missing value. Empty
initial commands are never inferred enabled. Command result carries no optimistic
Value; the participant emits a separate snapshot. Text label/gap/padding/font
measurement rules are in SDP-07-02. Font signature includes family, size, language
and measurement implementation; change forces new preparation. Canonical metrics
are in SVG px and are shared by native text-field font setup.

Proposed C++ codec is nlohmann/json 3.12.0, header-only, MIT, upstream commit
`55f93686c01528224f448c19128836e7df245f72` (tag resolved through official GitHub
API 2026-09-18). Vendor/pin and install its license in XFMD during X1; no download
at runtime. This is a new build dependency, not already present in XFMD. Rust uses
existing serde/serde_json. Both sides reject duplicate keys with an explicit
per-object key check; a default DOM parser accepting duplicates is insufficient.
Bound bytes/depth before materialization, reject unknown fields, and validate all
semantic constraints. C++ JSON types stay inside the ABI adapter, not contracts.

Parse success envelope is `{contract, model, childSources, diagnostics}`; parse failure carries
`{contract, diagnostics}` and nonzero ABI status. Prepare success is the frame
schema; failure carries the same diagnostic envelope. Every diagnostic may include
`sourceStart`/`sourceEnd` uint64 byte offsets (both or neither); offset overflow or
out-of-source range rejects decoding. Status values: 0 success, 1 invalid input,
2 unsupported version/capability, 3 resource/budget, 4 cancelled, 5 internal error.
No partial success model on parent error. The bridge owns zero-terminated-free byte spans:
strings need not contain a trailing NUL; size is authoritative. Results are bounded
by the stated frame limit even on errors. Panic maps to 5 without unwinding over C.

## 10. Embedded-diagram interpretation boundary

Source schema is authoring; normalized model schema replaces each diagram.source
with childRef (equal to that node ID). Parse returns a bounded childSources array
with ref, family, source UTF-8 and half-open sourceStart/sourceEnd. This extraction
is not diagram interpretation. XFMD interpreter consumes each source through its
existing IDiagramInterpreter and stores a typed DiagramModel or local diagnostic
beside the parent BoxUiModel. No child source is passed to BoxUI layout/prepare.

The application preparation chain sends typed child models to the existing
IDiagramLayout using the same metrics/palette and remaining budget. Child intrinsic
geometry is prepared once; parent diagram minimum size does not depend on it.
BX-HOST prepare.children contains exactly one `{ref,width,height,svg}` or
`{ref,error}` for every childRef, no extras/duplicates. The parent library only
contain-scales/clips/namespaces these child SVGs. Never scrape child SVG for bindings
or use external image href. This preserves layout independence and avoids a
parse/layout/reflow cycle. A child error remains local.

The standalone Rust BoxUI convenience API can orchestrate its own parser/child
layout before prepare, but `prepare_boxui` itself has no source-parsing callback.
Child profile capability names are identifiers agreed with the host, not a claim
of universal upstream syntax compatibility. No future callback can silently cross
this boundary. The model/prepare fixture illustrates this split with a marked mock
child SVG; it is not actual Mermaid rendering evidence.

Codec provenance: [nlohmann/json 3.12.0 source](https://github.com/nlohmann/json/tree/55f93686c01528224f448c19128836e7df245f72),
[MIT license at that revision](https://github.com/nlohmann/json/blob/55f93686c01528224f448c19128836e7df245f72/LICENSE.MIT).

Host retains the immutable prepare snapshot alongside the accepted frame key; native
input initial text comes from its valueBinding in that snapshot, not SVG parsing.
Control semantic metadata includes all input surfaces (kind=input); no second
geometric map is independently recalculated. On publish, the retained draft wins
over initial accepted text only under the compatibility rules in section 5.
