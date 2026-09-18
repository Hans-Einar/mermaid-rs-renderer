---
document_id: SDP-07-02
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Presentation and widgets

## Candidate first profile

Required first widget roles: value/text display, button, text input and diagram
pane; initial layout policies: nested row and column with explicit sizing.
The exact grammar is not fixed. Standard Mermaid diagrams remain child content,
and BoxUI has its own versioned syntax identity.

A widget definition declares properties/types, semantic events, measurement,
visual states, focus/keyboard behavior, validation and lifecycle. Instances have
stable IDs and binding references. A compiled registry is a candidate extension
mechanism; arbitrary source-provided executable plugins are not implied.

## Layout and frame contracts

Resolve intrinsic/min/max/flexible sizing, spacing, clipping, overflow and behavior
at a too-small viewport. Preserve editing drafts and focus across compatible
changes. A changing Value must not become a layout weight unless declared.

Prepared output must associate geometry, visuals, focus order and input bindings
with one revision. Host input refers to the visible frame; publication rejects
obsolete preparation. Text input requires a real editing/input mechanism, not
merely an SVG rectangle with a label.

## Embedded diagrams

A diagram pane references content plus the supported Mermaid family/profile,
scale/clip policy, diagnostics and rendering budget. Define source-ID scoping.
Nested BoxUI and recursively embedded content remain open and must be bounded.

Static depiction, interaction semantics and domain outcomes have separate
acceptance evidence. A disabled visual control does not establish domain permission.

