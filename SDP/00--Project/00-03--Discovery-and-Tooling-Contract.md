---
document_id: SDP-00-03
profile: sdp-system-development-pilot/0.1
status: pilot
updated: 2026-09-18
---

# Discovery and tooling contract

## One project entry

A future tool receives the path to the project's **SDP directory**, opens
`sdp-project.json`, checks its `schemaVersion` and `profile`, and resolves
the explicitly listed records. The accompanying
[schema](sdp-project.schema.json) defines the pilot's JSON shape.

This is a proposed discovery contract. No existing SDL CLI or SDP-Analyzer is
claimed to accept it. Do not document invented runnable commands as implemented.

## Path domains

Document, descriptor, SDL member and entry paths are relative to SDP root.
They must be canonical relative paths without `..`, absolute paths or symlink
escape. Generated output paths are relative to `generated/`; they are not sources.
Source bindings use a separate explicit **repository** root, the parent of SDP,
and repository-relative paths such as `src/parser.rs`. Resolve real paths and
reject escapes from each root. External study links are provenance, not implicit
compile members.

## Records and responsibilities

| Record | Required meaning |
|---|---|
| documents | Stable slot ID, canonical path, title and document status. |
| modules | Stable component identity, kind, existing/planned implementation, defining document, explicit source bindings and evidence references. A module is not automatically a Container. |
| sourceSets | Language/profile, explicit members and model entry points. An empty list means no SDL model is declared yet. |
| targets | ID, kind, module, prerequisites, state, runner binding or explicit null, and output paths. |
| blueprints | ID and descriptor path. Descriptor contains view scope, source identities, projection rules, generator target and expected output. |
| relations | Typed links among registered project records, with rationale and evidence status. |
| generatedRoot | Dedicated output location, never automatically scanned into source sets. |

Documents own narrative contracts; the manifest owns discoverable inventory.
Blueprint JSON owns machine selection/output facts; its numbered Markdown
companion explains purpose, preservation rules and limitations. Do not parse
prose headings as a replacement for a missing structured declaration.

## Listing and execution are separate

Listing must show **all** registered targets with readiness and reasons, including
planned and blocked targets. The existing renderer test and scoped design-artifact checker have runner
bindings in this pilot; its availability is not evidence that its tests pass.

A native runner uses an argv array and an explicit working-root identity, never
shell interpolation inferred from Markdown. Discovery and Analyzer rendering
must not execute targets, embedded code or recorded evidence commands. Explicit
execution belongs to a separately invoked runner under project authorization.

A future SDL driver selects a target, resolves source/profile and transitive
dependencies, reports unresolved contracts/bindings, then parses/links/checks.
It lowers only executable-complete selected slices. A structural blueprint may
need less than an interpreter run. Known simulated bindings must remain visible.

## Blueprint description contract

Each descriptor declares stable ID, explanatory document, abstraction level,
root module, requirement scope, source documents/source sets, output format,
projection-rule reference, generator target and output path.
Source identities/revisions and generator/profile versions must be attached to
generated results. A pretty diagram is not evidence of model conformance.

Initial descriptors cover system responsibilities, widget layout and input/frame
interaction. All generation targets remain blocked: structural SDL exists, but behavioral/widget
semantics and projection/generator bindings are still missing. Hand-authored explanatory diagrams are labelled as such, not generated.

## Proposed SDP-Analyzer adaptation

At inspected revision `632991a878100e8cd8c4efbb7d724edb3694d98a`,
[SDP-Analyzer](https://github.com/Hans-Einar/SDP-Analyzer/blob/632991a878100e8cd8c4efbb7d724edb3694d98a/README.md)
analyzes three old structured traceability files and discovers Markdown paths
without reading their contents. Its browser-directory adapter is not an
implemented website generator for this profile.

Add a versioned adapter rather than silently reinterpreting the old profile.
The future site should expose: phase/document navigation, requirements and
relations, modules, runnable/testable targets with blockers, blueprint catalog,
decisions, evidence and provenance. Unknown versions, duplicate identities,
broken links, unavailable sources and stale evidence remain visible findings.

Render source content inertly. Do not auto-run target commands or confuse a
`verifies` relation with a passed result. Evidence needs its exact subject,
revision, check, observed outcome and artifact reference.


## XFMD versus future Analyzer UI

The future Analyzer site discussed above is separate tooling. It is not the BoxUI
prototype host. The required host is native FOX/XFMD displaying interactive
BoxUI inside Markdown; HTML/CSS/browser implementation is not this delivery.
