---
document_id: SDP-06-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Internal layers and contracts

## Proposed implementation roles (none implemented)

| Layer / proposed path | Single responsibility | Public operation |
|---|---|---|
| `src/boxui/model.rs` | immutable typed authoring tree and validated binding declarations | `BoxUiDocument` |
| `src/boxui/parse.rs` | version/header + strict JSON, spans, diagnostics | `parse_boxui` |
| `src/boxui/validate.rs` | IDs, typed properties, binding references and budgets | `validate_boxui` |
| `src/boxui/registry.rs` | built-in widget definitions and host capability matching | `WidgetRegistry` |
| `src/boxui/layout.rs` | deterministic row/column allocation using supplied text metrics | `layout_boxui` |
| `src/boxui/svg.rs` | complete static SVG and preview SVG edit surfaces | `render_boxui` |
| `src/boxui/frame.rs` | immutable frame/control map and diagnostics | `prepare_boxui` |
| `src/boxui/embedded.rs` | composition of prepared child SVG/errors; no child parsing | `compose_child` |

Public Rust operations return typed Result; no parser text accepted by layout.
Registry is immutable after construction and scoped to the call/session, never a
mutable global. Built-ins text/1, value/1, button/1, input/1, diagram/1 declare
property/value types, measure/paint behavior, role and accepted semantic actions.
Extensions are compiled trusted Rust registrations, not document-provided code,
dynamic libraries or arbitrary native FOX classes. Unknown kind/version rejects
preparation with a located diagnostic. A future registry addition also needs a
host capability contract for new interaction semantics.

Model owns strings/data; frames are immutable shared values. No FOX/Cairo handles
cross the library boundary. Text measurement is a borrowed synchronous callback
on the preparation worker and is never retained. No GUI callbacks there. Calls
use independent state; no mutable layout/registry shared across worker threads.
Rust panic is caught by the XFMD bridge; C++ exceptions are caught before callbacks
return. Allocation abort/OOM is not promised recoverable.

The library outputs authoritative SVG and control geometry under one key.
Do not derive widget identities by scraping SVG elements. Drafts and focus are
application state and do not enter shared global diagram caches. No runtime
binding or domain command is executed while parsing, laying out or exporting.
