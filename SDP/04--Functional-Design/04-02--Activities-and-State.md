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


## Draft1 state transitions and witness

BX-A01: idle -> preparing -> ready -> published. New request supersedes pending;
prepare failure/old key -> discarded with old active frame retained. Publication
validates exact source/binding/state/viewport/theme key. Prepared is not presented.
A source edit marks the old frame stale immediately; controls cannot issue new
commands until matching publication. Mere resize leaves compatible old input usable.

BX-A02: clean -> editing -> pending -> accepted/rejected/unknown. Typing changes only
local draft; Enter creates one command, blur never does. Rejected keeps draft plus
reason; accepted observation replaces value only when correlated to same binding/
context; unknown requires explicit reconciliation, never automatic resubmission.
Read-only observation updates cannot overwrite a dirty draft. Removal cancels it.

BX-A03 synthetic fixture starts Activity A1, acceptedProgress=12, context C1,
state running. Suspend -> suspended preserves A1/progress. Resume under C1 ->
running same instance. If source context becomes C2 while suspended, an old-frame C1 submission is rejected by the host before dispatch. A new
C2 Resume reaches the participant, which rejects because A1 retains C1; A1 stays
suspended/progress=12. These are separate host-currentness and domain checks. Explicit start
under C2 creates A2 with progress=0; this is a distinct action, not Resume. That
new-start action is an oracle fixture action, not an extra first-profile button.
Late observation from the old source session never makes a stale value current.

Scenario driver, simulated participant and assertion oracle are distinct roles.
The first driver is an XFMD-owned test fixture using a controlled tick counter;
there is no new scenario DSL interpreter. User enters prototype mode explicitly;
it installs only this known synthetic participant. No Markdown-provided code runs.
Closing document ends session and subscriptions; reopen starts a new nonce.
