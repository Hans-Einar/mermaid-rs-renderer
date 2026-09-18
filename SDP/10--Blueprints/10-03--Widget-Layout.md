---
document_id: SDP-10-03
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# Widget layout blueprint

## Purpose and projection rules

BX-BP02 renders the first selected BoxUI presentation with region hierarchy,
widget IDs and binding metadata. It supports static inspection and, with an
explicit host binding, interactive exploration.

Preserve sizing/overflow rules, viewport identity and diagram-pane bounds.
A treemap-style area view is an optional separate projection, not a substitute
for the BoxUI layout contract. Value changes must not silently alter geometry.

Output format/profile is planned BoxUI source/scene; no existing tool is claimed
to understand it. Record source/model and profile versions with each generated
artifact. Check source-to-widget identity and frame/input agreement.

