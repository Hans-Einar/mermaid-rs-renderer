# Ubuntu-målinger for sporbarhetsgrafen

Generert 2026-09-16 i Ubuntu 24.04-container fra image config
`6232b38791000e3818b58d8847b5a8f5612d606929e01156dd8febc423e0f2ef`.
Pango 1.52.1, fonts-dejavu-core, DejaVu Sans 12 pt, Cairo FontMap 72 dpi,
`CAIRO_HINT_METRICS_OFF`, single-paragraph-mode. `get_size()` deles på
PANGO_SCALE og 0,75, akkurat som XFMDs layout-wire. Labels er de unike node-
og kanttekstene i traceability.mmd, inkludert tom tekst.

Disse målene reproduserte CI-feilen nøyaktig på en annen Linux-distribusjon:
`ambiguous label on edge 1: own distance 24.1, other (16, 17.0047)`.
Testen var rød før siste etikettpass og grønn etter. Den krever ikke Pango i
Rust-testmiljøet; tallene er et eksplisitt frosset grensesnittdatasett.
