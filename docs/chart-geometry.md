# Chart geometry corrections

Sankey now sizes rank gaps from actual caption widths and emits labels after
translucent links, using the measured font family. Kanban reserves the rendered
heading inset before the first card. Small shape port candidates are deduplicated:
initial/final state circles otherwise create identical exclusive pins when the
available tangential span is zero. This removes one cause of unstable routing;
byte-identical output is still not guaranteed for all competing-pin graphs.

Evidence: 398 library tests, 12 libavoid tests, three planning and three semantic
profile tests, two chart geometry tests. Semantic currentness repeats 24 times
and checks edge identity, caption content and orthogonality. Native errors are
returned explicitly; no repair pass silently accepts nonorthogonal routes.
