---
document_id: SDP-03-01
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# User needs and use cases

## Needs and optional stories

As a designer, I want to describe and try a proposed UI alongside its system
behavior so that layout and interaction gaps emerge before complete implementation.
As a reader/agent, I want predictable SDP entry documents so that relevant design
intent does not require reconstructing old issue discussions.

## Initial Use Cases

| ID | Actor goal | Observable outcome |
|---|---|---|
| BX-UC-01 | Author and inspect a BoxUI layout | Regions, widgets and diagrams are readable; invalid input is diagnosed. |
| BX-UC-02 | Interact with a modeled Activity | Typed input reaches the intended binding; current state and outcomes are visible. |
| BX-UC-03 | Replace a presentation during work | New frame appears coherently and retained controls preserve declared interaction state. |
| BX-UC-04 | Review design and verification coverage | Requirements, modules, blueprint targets and evidence can be found from SDP root. |

Stories are optional context. Requirements and Use Cases may exist independently.
These cases propose scope within the mandate; exact actors, preconditions,
alternative paths and measurable acceptance criteria must be completed before
claiming design coverage. A UI demonstration alone does not prove every case.


## Concrete first actor and paths

The actor opens a local `.md` file in XFMD; no web page is launched.
BX-UC-01 precondition: supported fence/profile; success is readable static graphics
and available controls. Unknown profile/duplicate block ID gives a local source diagnostic.
BX-UC-02 precondition: explicitly enabled registered synthetic session; mouse and
keyboard submissions reach its typed port, results and Value currentness are visible.
Unbound controls remain disabled; opening a document alone never starts a scenario.
BX-UC-03 precondition: dirty input draft and active frame; resize/theme-only replacement
retains it, binding/context changes reject stale submissions and explain discarded drafts.
BX-UC-04 starts at SDP README and reaches source model, contracts, acceptance cases,
missing runtime bindings and separate XFMD task without implying those targets ran.
