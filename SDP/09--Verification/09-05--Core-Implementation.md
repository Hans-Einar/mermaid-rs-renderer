# R1/R2 — BoxUI-kjerne, fase 045

Implementasjon autorisert av brukeren etter gjennomgang av kontraktgrunnlaget.
`src/boxui/model.rs` eier separate typede widgetverdier; `parse.rs` validerer
strict JSON og trekker ut child-kilder med byteområder. `layout.rs` plasserer
row/column og widgets med målte tekstbredder. `frame.rs` eier interne SVG-/control-
verdier, `svg.rs` produserer separate statiske/preview-varianter. Dette er crate-API;
BX-HOST-envelope, nøkler, snapshot-lister og ABI-status bygges i XFMD-adapteren.
Ingen FOX, domeneutførelse eller Mermaid-kildeparsing i BoxUI-layout.

Fasekontroll 2026-09-18:
`cargo test --locked --no-default-features --features libavoid --lib boxui`
bestod 6 tester: typed ekstraksjon, duplicate/ukjent/typefeil, determinisme,
no-space/avbrudd, XML-escaping og ugyldig viewport. Tester bruker faste mål;
reell Pango/FOX/PDF-verifikasjon kommer i XFMD-fasene. Ingen GUI-dekning hevdes.
Avbrudd/deadline sjekkes mellom arbeidstrinn; ingen hard tidsgrense.

Filer er delt etter modell, parsing, framekontrakt, plassering og SVG-generering.
De planlagte kildebindingene i SDL er historisk design; dette dokumentet angir
første faktiske implementasjon og skal suppleres av vertens testbevis.
