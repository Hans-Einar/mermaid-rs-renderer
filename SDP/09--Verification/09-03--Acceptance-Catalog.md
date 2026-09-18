---
document_id: SDP-09-03
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Acceptance catalog — planned product checks

None of these product checks has run. Schema/parser authoring checks are separate.

| Case | Requirements | Stimulus and expected observation | Primary owner |
|---|---|---|---|
| BX-AT01 | R01/09/13 | mixed `.md` with both fence aliases, ordinary code and links; same BoxUI tree; `.txt` literal; no browser/network process | XFMD |
| BX-AT02 | R01/04/08 | duplicate JSON key/ID, unknown kind/version/property, wrong binding type/role, malformed UTF-8; located error, no partial semantic tree | renderer |
| BX-AT03 | R02/03/08 | 320x240/640x480/1280x720, long/Unicode labels, depth/bytes boundary±1, invalid child; no NaN/overlap, explicit no-space or local child error | renderer |
| BX-AT04 | R04/12/13 | mouse and keyboard, repeated Space/Enter, disabled button, paste æøå/日本語; one typed command, normal clipboard and visible focus | XFMD |
| BX-AT05 | R05/06/07 | dirty draft during resize/theme then binding replacement; first retained, second cancelled with notice; no unsolicited submit | XFMD |
| BX-AT06 | R06/07/13 | capture press on frame1, publish frame2 before release; no dispatch. Source edit during prepare rejects old action; stale result never publishes | both |
| BX-AT07 | R04/06/10 | A1 suspend/resume same context, then changed context reject; independent oracle checks identity/progress/outcome, duplicate command returns ledger result | XFMD synthetic participant |
| BX-AT08 | R09/14 | PDF during dirty edit/pending result; frozen accepted values, explicit simulated marker, vector SVG, no native controls or events | XFMD |
| BX-AT09 | R02/12/13 | Wrap/A4, scroll, zoom 100/150/200%, partial clip; native field and hit rect track same geometry within 1 device px; off-clip area cannot activate | XFMD |
| BX-AT10 | R06/08 | 65th queued command and 4097th session command refuse visibly; pending updates coalesce, commands never disappear; close releases worker/overlay/ledger | both |
| BX-AT11 | R09 | existing 23-family XFMD fixture path and fork regressions, with/without BoxUI feature, offline after bootstrap, sanitizers/FFI ownership | both |
| BX-AT12 | R11/15 | manifest/schema/SDL canonical check, source mapping, positive/negative wire fixtures and mock/real distinction; review accepts shared revision | design gate |

Rxx denotes BX-Rxx. Required failure cases also include child 9, node 257,
negative/infinite geometry, clip escape, incompatible key/version and missing host
capability. Corpus owner records exact revision, command, outcome and artifact.
Visual review must inspect actual preview and PDF in Light/Dark and reading colors;
XML validity and mock frames never substitute. IME composition/AT-SPI limits are
reported per environment, not silently labelled universal accessibility support.
