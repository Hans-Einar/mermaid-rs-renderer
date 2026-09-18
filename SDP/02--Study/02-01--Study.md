---
document_id: SDP-02-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Study and evidence baseline

## Inspected facts

- Renderer workspace HEAD: `225a5a28632bbfd36c78fa522327f5dbe177d78e`.
  `src/ir.rs` lists 23 DiagramKind variants without BoxUI.
  `src/parser.rs` dispatches Treemap separately; `src/layout/mod.rs` has a
  treemap layout path. `src/lib.rs` exposes parse/layout/render stages.
  These are reuse points to study, not a decision to squeeze BoxUI into Graph.
- `Cargo.toml` defines the Rust library and optional CLI, PNG and scene features.
  No BoxUI implementation was found by the initial scoped source search.
- The existing SDP note had prepared this branch for a mandate and cited older
  SDL research; its meaning and baseline references are summarized in
  [the workspace provenance record](02-02--Workspace-Provenance.md), with the
  original retained in Git at the inspected HEAD.
- Current checkpoint work is the newer conceptual input, including Activity,
  Value/Command separation and the interactive prototype study. It remains
  a discussion baseline, not implemented grammar.
- SDP-Analyzer revision `632991a878100e8cd8c4efbb7d724edb3694d98a` reads the old
  structured traceability core; new manifest/Markdown/blueprint presentation
  requires an explicit adapter.

## Sources

- Local current discussion:
  [SDP checkpoint](../../../SDP/docs/checkpoint%231/README.md),
  [interactive prototype study](../../../SDP/docs/checkpoint%231/06-Interactive-Prototype-and-Renderer-Study.md).
- Earlier research:
  [SDL research entry](../../../SDP-research-issue-10-system-design-/SystemDesignLanguage/README.md)
  and [mandate/study](../../../SDP-research-issue-10-system-design-/SystemDesignLanguage/Mandate-and-Study.md).
- [Legacy SDP document guide](../../../SDP/SDP-DOCUMENT-GUIDE.md).
- [Analyzer README at inspected revision](https://github.com/Hans-Einar/SDP-Analyzer/blob/632991a878100e8cd8c4efbb7d724edb3694d98a/README.md).
- [XFMD coverage](../../../xfmd/mermaid_coverage.md).
  Its existing diagram support does not implement BoxUI interaction.

Local sibling-repository links are development provenance, not portable compile
dependencies. This mandate remains understandable without fetching those sources.

## Required investigations before detailed implementation

Compare a dedicated BoxUI AST/model with extending the existing Graph/scene types.
Inspect actual BOX/BoxUI contracts before claiming compatibility. Compare row/
column/grid constraints with weighted treemap layout for real controls. Work one
widget's parse-to-scene-to-input path. Identify host needs for text editing,
focus, hit testing, accessibility and frame publication.

Use alternatives and concrete fixtures, not only an attractive diagram. Record
which assumptions are confirmed, proposed or unknown, and promote selected
decisions with their impact and rejected alternatives.
