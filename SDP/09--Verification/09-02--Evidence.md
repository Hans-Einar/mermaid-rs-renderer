---
document_id: SDP-09-02
profile: sdp-system-development-pilot/0.1
status: evidence
updated: 2026-09-18
---

# Verification evidence

Current renderer implementation evidence is under **Phase 046 implementation
evidence** below. BX-E01 and the draft1 design checks are preserved historical
records; their design-only statements do not describe the current source tree.

## Delivery record BX-E01

Scope: mandate and numbered SDP discovery pilot, 2026-09-18.
No BoxUI source, product behavior, SDL parser or Analyzer implementation changed.

The authoring checks below were run against the uncommitted pilot records after
creation. The inspected repository HEAD remains the baseline, not a claim that
these new files are committed or published.

No renderer/Cargo tests, interactive-host checks, network runs or independent
implementation review are claimed. This evidence does not satisfy BX-R01–BX-R10
or BX-R12. It covers a bounded authoring instance of BX-R11, not an implemented
discovery tool or standard adoption.

## Authoring checks performed

Environment: local Python 3 with the installed `jsonschema` package;
`Draft202012Validator.check_schema` and `Draft202012Validator.validate`.
Additional read-only Python checks resolved identities, paths and local links.
The checks were executed during authoring, not installed as a new project CLI.

| Check | Observed result |
|---|---|
| Manifest schema and three blueprint descriptor schemas | Passed. |
| Canonical document IDs, paths, metadata and status | 25 resolved, no duplicate registered IDs. |
| Module and target references | Six modules and ten targets resolved. |
| Runner readiness distinction | One bound existing test invocation; nine blocked targets with null runners. |
| Target dependencies and blueprint outputs | References resolve; dependency graph acyclic; descriptor outputs match their targets. |
| Typed catalog relations | Eight resolve to registered identities. |
| Repository source binding paths | Existing renderer paths resolve inside repository root. |
| Local Markdown links | 60 resolve, including explicit sibling-repository study references. |
| Markdown fences | Paired. |
| Invalid schema version and unknown root field | Rejected in negative schema checks. |
| Blocked target with runner; bound target without runner | Both rejected in negative schema checks. |
| Document path containing parent traversal | Rejected in negative schema check. |

Manifest SHA-256 at check:
`c1851a4d89c70b7a5d02519d92666face9493f456695d00766f9c43e88fb1d75`.
Schema SHA-256 at check:
`9a5590d9ae6febff7e0f8556a7bd1baa387323c79665c3102bce75e461628ba6`.

These checks establish consistency of this authoring snapshot. They are not a
general validator implementation, a passed Cargo test suite, executable SDL
coverage or evidence that the proposed structure is optimal for other projects.


## Delivery record BX-E02 — parallel-design review checkpoint

This additive record supersedes BX-E01 counts/current-path inventory; BX-E01
remains historical authoring evidence from before mandate relocation.

Checked draft1: 28 documents, one real design-core 0.1 source set, 11 targets,
local Markdown links (count in log), six contract schemas/positive fixtures, ten negative
artifact cases. Source/model/prepare fixture consistency is checked separately
from rendering. [Actual checker output](evidence/design-check.txt).

The external SDP design-core parser accepts the canonical structural model with
zero diagnostics; reversed ownership is rejected. [Positive](evidence/sdl-check.json),
[negative](evidence/sdl-negative.json), [source/parser fingerprints](evidence/design-fingerprint.json).
These checks establish syntax, structural typing and bounded artifact consistency.
They do not establish temporal behavior, native input, SVG safety, layout quality,
PDF output, thread safety or source-code conformance. No product tests or build ran.

Manual author review traced parser -> typed model -> child preparation -> layout
-> frame publication -> FOX controls -> intent -> participant -> observed Value.
It found and corrected (1) optional-XFMD wording, (2) raw child-source parsing at
the layout boundary, (3) distinction between old-context host rejection and
participant context conflict, (4) mock viewport mismatch. This is author review,
not an independent verifier or owner approval.

Gate G-PARALLEL-REVIEW: design package available for recap. No implementation gate
has passed. Remaining bounded choices are recorded in SDP-12-01.
# Phase 046 implementation evidence

R1: strict versioned source parsing, normalized typed models, child-source byte
ranges, immutable built-in registry and semantic validation implemented. The
command `cargo test --locked --no-default-features --test boxui` passed 3 tests
covering the shared source/model fixture, duplicate keys/IDs, forbidden/null
fields, incompatible bindings, unsupported versions, trailing bytes and UTF-8.
This is parser evidence, not native-host interaction evidence.

R2: typed prepare decoding, snapshot/identity checks, borrowed measured layout,
two-pass wrapping, SVG/control geometry, static/native-overlay variants and
inert child composition implemented. The same BoxUI test command now passes
8 tests. `cargo run --locked --no-default-features --example boxui_demo`
produces a complete frame with an actual Mermaid state diagram. `rsvg-convert`
successfully rasterized both SVG variants; the static image was visually
inspected. The inherited state-diagram renderer places two transition labels
close together; this is not evidence of polished child-diagram layout.

The XML composition boundary uses explicit `roxmltree 0.20` (already present
transitively in the lockfile). It rejects DTDs and non-profile elements/attributes,
rewrites local ID references and isolates child failures. No raw child XML is
copied unchecked. Inline CSS is a restricted property/value subset; style blocks,
links, images, filters, animation and foreignObject are unsupported.

R3: 18 BoxUI integration tests pass both with default Cargo features and with
`--no-default-features`. They now cover exact byte/node/depth/text/child limits,
wrong/missing/duplicate snapshot entries, XML rejection and ID rewriting, all
three real child families, bounded frame size, deadlines, cancellation, parallel
session isolation, font fallback and truncated-source diagnostic ranges.

`python3 SDP/09--Verification/check_boxui_runtime.py` passed against freshly
built `boxui_host` and `boxui_demo`: real Rust model/frame payloads validate
against the unchanged draft1 schemas, replay deterministically and preserve
source byte ranges/frame keys. Three error envelopes also pass the expected
status checks. Approximate demo metrics do not establish host font parity.

`python3 SDP/09--Verification/check_design.py` passes with 29 registered
documents, one structural SDL source set, 11 targets, six schemas and ten
negative authoring checks. Two BoxUI targets now have actual runner bindings;
native interaction and SDL execution retain explicit blockers. The historical
design fingerprint files remain records of their earlier checkpoint.

The available toolchain is Rust 1.92.0. `cargo clippy` is unavailable in this
installation; no Clippy result is claimed. Scoped rustfmt and `git diff --check`
pass. No independent reviewer, sanitizer, C ABI or native interaction result is
claimed by this phase.
