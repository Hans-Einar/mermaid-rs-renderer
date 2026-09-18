---
document_id: SDP-10-02
profile: sdp-system-development-pilot/0.1
status: planned
updated: 2026-09-18
---

# System responsibilities blueprint

## Purpose and projection rules

BX-BP01 explains which roles belong to the renderer library, BoxUI extension,
host adapter and scenario collaborator. Show library membership separately from
runtime deployment. Do not turn every module into a Container or infer threads
from arrows. Include contract names and unresolved allocation.

The descriptor selects source documents and the root module. A future generator
must resolve explicit model facts; current prose is not executable SDL.
Target format is a Mermaid flowchart under a declared compatible profile.
Tests must check included identities, ownership/interaction labels and unresolved
links, not only whether a picture was produced.

Changing a source boundary invalidates the corresponding view and its evidence.

