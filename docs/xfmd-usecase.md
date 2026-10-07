# XFMD native use-case increment

Typed clients may construct DiagramKind::UseCase with UseCaseActor and Ellipse
node shapes and existing UML edge markers. XFMD owns its strict Mermaid parser;
this fork does not advertise source parser support for usecase-beta.

Native measured sizes reserve the actor glyph above its label and fit the label
rectangle within the ellipse. A 64-segment convex ellipse outline participates in
existing polygon port/intersection handling. Use-case rendering shares the UML
open arrow/triangle marker definitions. Boundaries use the existing one-level
subgraph geometry; nested boundaries are outside the XFMD profile.

compute_semantic_layout preserves the semantic family. Unbounded graphs use its
existing Libavoid route validation and single spacing retry. Graphs with boundaries
use the existing native composite layout, explicitly diagnosed; there is no retry
that switches from a failed backend to another engine. Cooperative deadlines remain.
Native/PDF consumer acceptance is separate from these library-level checks.
