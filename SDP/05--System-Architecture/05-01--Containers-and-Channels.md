---
document_id: SDP-05-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# System boundaries and channels

## Proposed baseline BX-HOST/0.1-draft1

XFMD is the first required runtime Container. The renderer crate and BoxUI module
are libraries within that process, not network services. An XFMD-owned synthetic
participant is replaceable through a domain port; its simulation policy never
belongs in the renderer. No new HTML/CSS UI, Node, browser or TCP service.

| Role / owner | Contract and state |
|---|---|
| XFMD interpreter | Markdown source -> immutable BoxUiDocument; no FOX or layout. |
| Renderer BoxUI library | Validate model, measure/layout, SVG and ControlMap; no focus, drafts or domain state. |
| XFMD renderer adapter | model/metrics -> BoxUiFrame; document placement, transforms and hit testing without FOX. |
| XFMD application Composition | Session, per-block instance identity, worker lifetime, registrations, active frames. |
| XFMD Presentation coordinator | Binding resolution, currentness, command correlation, draft reconciliation and event dispatch. |
| FOX host adapter | Input acquisition, native text editing, focus, clipboard, clipping, widget overlays and paint. |
| Synthetic participant | Activity instance, work context, observations, command outcomes; explicit simulated provenance. |

## Channels and authority

BX-C01 interpretation: source bytes -> model/diagnostics. BX-C02 preparation:
immutable model + snapshot + metrics -> sealed frame. BX-C03 interaction:
visible frame event -> typed intent -> explicit result. BX-C04 observation:
participant snapshot -> Presentation binding map. All are in-process contracts
in the first profile, not implied OS threads. Only preparation uses a worker;
FOX widgets and publication live on the GUI thread.

Future SDL execution replaces the synthetic participant through BX-C03/C04;
it does not replace the widget library or make Mermaid sequence arrows executable.
Future remote transport must preserve the same identity and publication rules,
but has no wire protocol, listener or reconnect promise in this increment.

## Reuse and boundaries

[Host contract](../06--Container-Design/06-02--XFMD-Host-Contract.md) is the shared
handoff. Existing Mermaid parse/layout/SVG entry points stay compatible.
BoxUI's dedicated model does not alter Graph or standard treemap semantics.
XFMD keeps FOX-free value contracts under src/contracts and source-independent
layout under src/renderer. PDFs consume a static complete SVG, not live controls.
