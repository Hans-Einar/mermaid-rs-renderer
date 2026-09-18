---
document_id: SDP-04-02
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Activities and state

## Candidate activities

BX-A01 **Prepare and publish presentation**: validate source and bindings, prepare
a frame, assess currentness, and publish or report rejection. State includes
staging identity, sealed source revision, viewport identity and active frame.

BX-A02 **Edit and submit a value**: preserve a local draft, validate it, emit
typed intent, and present the correlated result. A repaint does not commit input.

BX-A03 **Run the synthetic Activity scenario**: publish observations, suspend,
revalidate and resume, or complete/cancel with outcome-specific postconditions.
This is the interpreter/sequencer collaborator's responsibility.

## Lifecycle obligations

Each definition needs entry conditions, triggers, invariants, allowed effects,
iteration/progress meaning, suspension/resumption conditions and per-outcome
postconditions. An instance has stable identity and accepted progress.
Resume continues an instance after revalidation; a new start is not a rewind.

Scenario paths must include invalid source, changed work context, late frame
completion, duplicate event, removed widget and input while a new frame prepares.
State describes scoped knowledge with owners/revisions, not a frozen global snapshot.

This is a design inventory. No SDL Activity execution semantics or state-machine
implementation is supplied by this document.

