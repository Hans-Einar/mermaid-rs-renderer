---
document_id: SDP-08-03
profile: sdp-system-development-pilot/0.1
status: inventory
updated: 2026-09-18
---

# Renderer API and XFMD handoff

## Phase and integration point

Branch `phase/boxui-046-implementation`, based on design handoff `7076cac`.
This is an additive Rust library implementation of `boxui/0.1` and
`BX-HOST/0.1-draft1`. The owner authorized implementation in BX-D15. It does
not implement the XFMD C bridge, FOX widgets, event ledger or frame publication.
Those remain the parallel XFMD workstream's responsibility.

The original phase-045 worktree had uncommitted files when this phase started.
This implementation lives in the sibling `mermaid-rs-renderer-boxui-implementation`
worktree to preserve them. Integrate the phase-046 commits as one implementation;
do not combine similarly named unfinished parser files from both branches.

Concept1's UIBox is React JSX/props/CSS. It does not currently consume this JSON
profile. Sharing source descriptions requires an explicit React adapter; this
phase does not change Concept1 or claim that adapter exists.

## Public Rust seam

Import `mermaid_rs_renderer::boxui::*`. No optional Cargo feature is required.
The normal Mermaid parser, graph IR, renderer and treemap remain independent.

| Function/type | Purpose and boundary |
|---|---|
| `parse_boxui(&str)` / `parse_boxui_bytes(&[u8])` | Version header and strict JSON to `ParsedBoxUi`; fixed resource limits; no child interpretation. |
| `ParsedBoxUi` | Contract, normalized `BoxUiDocument`, extracted `ChildSource` list and diagnostics; serializable as the agreed parse envelope. |
| `validate_boxui(&BoxUiDocument)` | Recheck model structure, widgets, identities, sizes and bindings. |
| `WidgetRegistry::BUILTINS` / `supports` | Fixed seven-kind registry, version 1; no dynamic plugins or fallback widget. |
| `decode_prepare_json(&[u8])` | Strict duplicate-key/unknown-field/null checks and semantic validation at an untrusted wire boundary. |
| `PrepareRequest` | Exact draft1 key, model, snapshot, viewport, palette, font signature, budget, capability names and prepared children. |
| `TextMetrics` | Borrowed synchronous `measure`, `font_family`, `font_size`; actual host metrics in logical SVG pixels. |
| `layout_boxui(&PrepareRequest, &dyn TextMetrics)` | Measured geometry and wrapped label lines for diagnostics/debugging. |
| `prepare_boxui(&PrepareRequest, &dyn TextMetrics)` | Atomic `BoxUiFrame`: static SVG, preview SVG, ordered control geometry, diagnostics and unchanged key. |
| `prepare_boxui_cancellable(request, metrics, &dyn Fn() -> bool)` | Same output with cooperative deadline/cancellation polling. |
| `BoxUiError` / `status_code()` | One bounded diagnostic and its draft1 status category. Bridge serializes `{contract, diagnostics:[diagnostic]}`. |

Use `decode_prepare_json` at the C bridge boundary. Plain serde deserialization
does not reject duplicate object keys or distinguish absent optional fields from
explicit null. Typed in-process preparation revalidates semantic constraints.
Models are owned values; callers may clone/edit them, but a preparation borrows
an immutable model/snapshot and never retains host references after returning.

Example adapter core:

```rust
use mermaid_rs_renderer::boxui::*;

fn prepare_wire(bytes: &[u8], metrics: &dyn TextMetrics) -> Result<BoxUiFrame> {
    let request = decode_prepare_json(bytes)?;
    prepare_boxui(&request, metrics)
}
```

Host metrics must be deterministic for one `fontSignature` and return finite
width, positive line height and baseline within that height. Use the same font
in native edit overlays. A callback must return promptly; cancellation cannot
interrupt an in-process callback. `MonospaceMetrics` is a deterministic test/demo
approximation, not a production font measurement implementation.

Diagnostics locate a known node where available; otherwise malformed model
shape uses the complete JSON body range. Ranges refer to the original UTF-8
bytes, including the source header offset. A child-source range includes its
JSON string token, not just the unescaped child text.

## Frame composition and failure semantics

Each child reference requires one prepared SVG or local error. Parent prepare
never invokes a Mermaid parser. `CHILD_PROFILES` exposes the three frozen host
capability identifiers from draft1. Missing capabilities and unsupported child
SVG constructs produce local pane diagnostics; valid sibling controls survive.
Malformed parent requests, identity/type mismatch, insufficient viewport,
invalid metrics, cancellation and resource overflow produce no partial frame.

The SVG subset accepts shapes, text, groups, markers, gradients and clipping.
It rejects DTDs, scripts, event attributes, external resources, links/images,
foreignObject, animation, filters, CSS style blocks and unknown attributes.
Restricted inline styles are accepted. IDs and exact local `url(#id)` references
are rewritten with pane-specific prefixes. Child Mermaid metadata is inert;
the parent never interprets it as a binding or event. An unsupported subset is
reported rather than silently losing content. This is a bounded composition
profile, not a general SVG sanitizer or universal Mermaid compatibility claim.

Input `Control.rect` and `Control.clip` describe the **edit surface**, excluding
its label/status. Static output paints accepted text; preview omits that text
and leaves the edit interior for FOX. The host initializes native text from the
retained snapshot using `valueBinding`. Disabled controls remain in the semantic
map; the host filters them from keyboard focus and rechecks enabled state at
dispatch. This renderer never submits a command or edits an accepted Value.

Regions use 8 px padding/gaps, source order, bounded weighted growth and at most
two layout passes. Inputs reserve a separate label and a minimum 32 px edit
surface, enlarged for the host line height. Simulated frames reserve a 24 px
minimum footer (enlarged for host metrics) for a visible `simulated snapshot`
marker in both variants. A layout that
cannot preserve content minima returns `layout-no-space`; it does not add scroll
bars or shrink controls below those minima. Root `size` has no parent main axis;
root geometry is the host viewport, less this explicit simulation footer.

## Wire compatibility and status mapping

The [JSON schemas](../06--Container-Design/contracts/README.md) remain unchanged.
Frame keys preserve canonical decimal u64 counters and exact session/block
identity. Every snapshot binding must resolve exactly once. Missing values use
null; current/stale values match the declared scalar type without coercion.
String input accepts only string value/string command bindings, and buttons
accept no-argument commands. Draft1 prepare supports string/none commands;
numeric/boolean command declarations cannot acquire a draft1 runtime binding.

Source limit is 256 KiB including header; extracted child text is at most 64 KiB
UTF-8. Text limits count Unicode scalar values. Prepare JSON is bounded to 8 MiB;
the serialized complete frame (both SVGs plus metadata) is also bounded to
8 MiB. XML has independent depth/node limits. Valid usable viewports start at
320 × 240 and end at 8192 × 8192. Budgets are cooperative, not a hard real-time
guarantee. Source decoding is bounded and synchronous; the bridge can check
cancellation before/after it. Preparation polls during measurement, allocation,
child processing and drawing, including after borrowed callbacks return.

Bridge status mapping: 1 invalid input/layout/metrics, 2 unsupported parent
profile/contract/version, 3 resource/deadline, 4 cancellation, 5 internal/panic.
A local child error is a successful parent frame with diagnostics, not ABI failure.
The bridge owns panic containment and C memory ownership; neither is supplied
by this Rust-only crate API.

## Reproduction and witnesses

Run from repository root:

```sh
cargo test --locked --no-default-features --test boxui
cargo run --locked --no-default-features --example boxui_demo
cargo build --locked --no-default-features --example boxui_host
target/debug/examples/boxui_host prepare < SDP/09--Verification/fixtures/prepare.json
python3 SDP/09--Verification/check_boxui_runtime.py
```

`boxui_host parse` takes the complete versioned source on stdin. Both commands
write JSON on stdout and diagnostics in the agreed envelope on failure. The
example uses approximate metrics and is a protocol witness, not a production
out-of-process host architecture.

`boxui_demo` generates `target/boxui-demo/{prepare.json,frame.json,static.svg,preview.svg}`.
It interprets the existing Activity fixture's Mermaid source before preparing the
parent. Generated files are reproducible outputs; the original `frame.mock.json`
remains explicitly hand-authored. Existing state-diagram transition-label layout
can be cramped in this example; BoxUI does not relayout child diagram internals.

## Remaining integration acceptance

One draft1 limitation remains explicit: provenance exists on commands, not on
Values. The simulation marker is therefore triggered by a simulated command
binding. A values-only simulated block cannot be identified from this payload;
the host must label that context outside the block until a coordinated contract
revision supplies the missing provenance. Do not infer it from an arbitrary
`sourceSession` name or invent a hidden command to make the marker appear.

XFMD must pin the verified implementation commit, adapt measured fonts, and
test native text editing/IME, exact transforms, focus, stale frames, intent
deduplication, accepted-value PDF snapshots and teardown. This renderer session
cannot establish those host behaviors. Standard treemap is unchanged. SDL code
generation, network streaming and a richer widget vocabulary remain future work.
See the [evidence record](../09--Verification/09-02--Evidence.md) for actual test
outcomes and limitations; the presence of a source binding is not acceptance.
