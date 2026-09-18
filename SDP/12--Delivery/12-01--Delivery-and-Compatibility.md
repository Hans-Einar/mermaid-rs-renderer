---
document_id: SDP-12-01
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# Delivery and compatibility

## Delivery at this checkpoint

Design handoff BX-HOST/0.1-draft1: Markdown source profile, structural SDL model,
contract schemas, manual boundary fixtures, role allocation, acceptance catalog
and a separate XFMD implementation task. Ready for recap, not a BoxUI release.
No runtime, new library API, interpreter or native widget behavior is delivered.

## Review questions before parallel start

1. Keep strict JSON under `boxui 0.1` for the bounded first authoring profile,
   or invest in concise textual syntax before implementation? JSON minimizes
   grammar work; it is not proposed as final SDL syntax.
2. Accept native single-line string input plus buttons/value/text/diagram panes,
   with tabs/collapse/grid deferred? Concept1 has more UIBox behavior than this.
3. Accept static accepted-value PDF snapshots and explicit built-in synthetic
   mode, without remote stream or executable scenario DSL in this increment?
4. Confirm numeric resource limits and native accessibility/IME scope after the
   first real FOX spike. Design includes failure behavior if those limits cannot hold.

These are bounded recommendations, not missing ownership/event semantics. If a
choice changes the shared boundary, revise schemas/fixtures and both work packages
before implementation. No schema promotion or SDP-wide vocabulary decision occurs here.

## Release conditions later

Record exact fork/XFMD commits, ABI/profile/registry versions, child capability
matrix, source snapshot, build dependencies, GUI/PDF evidence, sanitizer results
and known limitations. Publish one authoring guide tied to the supported subset.
Review profile changes without reinterpreting standard Mermaid/treemap. Native
implementation, simulation and unbound parts remain separately labelled.
