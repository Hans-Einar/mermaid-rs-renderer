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

## Focused investigation for the parallel handoff

Inspected on 2026-09-18; paths below are evidence, not new source bindings.

| Baseline | Actual observation | Design consequence |
|---|---|---|
| Renderer `225a5a2`, `src/ir.rs`, `src/parser.rs`, `src/layout/treemap.rs` | Graph/treemap has weighted nodes/edges, no widget lifetime or binding semantics. | Separate BoxUiDocument API/model; do not flatten widgets into Graph. |
| Renderer `src/scene.rs` | Optional scene uses SVG/usvg and outlined text; no native edit control. | Preserve SVG presentation plus a typed control map. |
| XFMD `c245fd9`, `MermaidBlockBuilder.cpp`, `SemanticDocument.h` | Only mermaid fences enter a DiagramModel path; no BoxUI block. | Separate BoxUI interpreter injection and semantic block. |
| XFMD `DiagramAbi.h`, `MermaidDiagramLayout.cpp` | Versioned, owned Rust result and synchronous measured-text callback; SVG px convert by .75. | Reuse ABI ownership pattern, not existing diagram payload. Explicit coordinate conversion. |
| XFMD `DiagramServices.cpp`, `DiagramPreparation.cpp`, `FoxPreviewInput.cpp` | Worker preparation, cancel/token checks, selection and link dispatch already exist. | Reuse scheduling; widget input precedes link/selection dispatch inside widget bounds. |
| Ponsse `882ad7c`, `Concept1/shared/ui-box/{README.md,model.mjs,UIBox.jsx}` | row/column weights, controlled/local state, stable page IDs, optional retained pages and collapse; React/CSS realization. | Reuse ownership/identity principles; no JS dependency or source port. Tabs/collapse are later coverage. |
| SDP `40add14`, `docs/Design-Language-Definition.md`, `experiments/design_core/` | design-core 0.1 checks structural Unit/Functionality relations; no executable state/widget semantics. | Supply a real bounded `.design` model plus explicitly separate contract records; do not invent accepted SDL syntax. |

Only Concept1 source was inspected; no implemented MVP1 BoxUI compatibility is
claimed. Current SDL checkpoint concepts guide naming, not executable support.
This study selects an additive Rust module and an XFMD adapter over (a) changing
weighted treemap, (b) converting all widgets to Graph nodes, or (c) embedding a
browser. Those alternatives lose semantics or conflict with the selected host.
