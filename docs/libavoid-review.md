# Libavoid: sammenligning og verifikasjon

Kjernelogikk: `ffc05d9aeb97288d39a0a263a12e0b9b7e2a646f`. Base: `f64ab1f`, upstream `3726ccb`.

Artefaktene er laget 2026-09-16 med standardtema, samme tekstmål og Current-nodeplassering. `fixed` beholder alle nodeposisjoner og mål; `end-to-end` plasserer på nytt og kan bruke én diagnostisert avstandsutvidelse. PNG er SVG rasterisert med librsvg, ikke en separat tegneimplementasjon.

| Graf / modus | Lengde gammel → ny | Svinger | Kryssinger | Parallelle overlapp | Tekstkontakter | Tid layout gammel → ny (ms) |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| traceability / fixed | 5119.5 → 4864.8 | 36 → 34 | 5 → 7 | 0 → 0 | 13 → 0 | 10609.6 → 207.2 |
| traceability / end-to-end | 5119.5 → 4864.8 | 36 → 34 | 5 → 7 | 0 → 0 | 13 → 0 | 10654.1 → 167.4 |
| service-map / fixed | 1997.0 → 2289.8 | 12 → 6 | 0 → 0 | 0 → 0 | 8 → 0 | 2594.8 → 18.0 |
| service-map / end-to-end | 1997.0 → 2289.8 | 12 → 6 | 0 → 0 | 0 → 0 | 8 → 0 | 2065.0 → 18.4 |
| layer-delivery / fixed | NoSpace, se diagnostikk | — | — | — | — | — |
| layer-delivery / end-to-end | 5000.4 → 7028.0 | 10 → 6 | 0 → 0 | 1 → 0 | 11 → 0 | 3333.4 → 66.4 |

Tider er én debug-kjøring på delt arbeidsmaskin, ikke en statistisk benchmark. JSON-filer inkluderer native rutetid separat. Ved avstandsretry inkluderer totaltiden det mislykkede forsøket; native rutetid viser transaksjonene i den vellykkede plasseringen.

## Observasjoner og begrensninger

- Sporbarhetsgrafen: kortere totalrute og færre svinger, ingen tekst-/nodegjennomganger. Flere kryssinger er akseptert. Det finnes fortsatt tre parallelle nærføringer under ønsket 8 px; diagnostikken rapporterer dette.
- Service-map: færre svinger, men noe lengre totalrute for å gi etikettene plass. Ingen universell påstand om korteste mulige ruter.
- Layer-delivery: frosne posisjoner gir NoSpace fordi en etikett blir nærmere feil kant. Ende-til-ende bruker én større avstand og lykkes; bildet blir bredere. Ingen skjult kollisjon leveres som suksess.
- Tekstkontakter teller også gammel ruting gjennom egen etikettboks; dette kan tidligere ha vært delvis skjult av etikettbakgrunn. Det er ikke identisk med bare tekst-over-tekst-kollisjoner.
- Nodegjennomganger er null i vellykkede nye eksempler. Portretning/formgrense valideres før resultatet publiseres; feil er eksplisitte Backend-feil.
- Gjentakelse: fem komplette kjøringer av de 11 fokuserte testene bestod, inkludert eksakte gjentakelsessjekker på ruter/etikettankre. Dette er ikke en garanti på tvers av plattformer; libavoid har fortsatt interne pekerbaserte tie-breaks.
- Gruppeinnhold er traverserbart og gruppetitler hindringer. Direkte forbindelser til selve gruppeboksen og endpoint labels avvises eksplisitt. Rektangel, avrundet rektangel, diamant og sirkel er verifisert.
- Avbrudd er kooperativt. Frist/callback testes før, under og etter transaksjon; ingen delrute publiseres. Ingen hard sanntidsgaranti.

## Visuell kontroll

Før/etter-SVG for traceability er undersøkt visuelt: gamle overlapp ved Unit/Container og tekstkontakt ved de lange etikettene er fjernet. Ny graf har tydelige separate inn-/utløp, men sentrale kryssinger og noen tettliggende segmenter gjenstår. Layer-delivery er undersøkt: tekst er leselig ved full størrelse, med ekstra bredde som eksplisitt kompromiss.

## Reproduksjon

```sh
python3 tools/verify_libavoid.py
cargo test --locked --no-default-features --features libavoid --test libavoid_suite --test measurements_suite
cargo run --locked --no-default-features --features libavoid --example libavoid_review -- tests/fixtures/flowchart/routing-review/traceability.mmd /tmp/traceability fixed
cargo run --locked --no-default-features --features libavoid --example crossing_review -- /tmp/crossing
rsvg-convert /tmp/traceability.svg -o /tmp/traceability.png
```

Se også `libavoid-integration.md` for kontrakt, eierskap, lisens og byggekrav.

## Utførte kontroller

- `ffc05d9`: 449 tester bestått i `release-fast`, opt-level=1, LTO av:
  380 library, 8 invariant, 28 layout, 11 libavoid, 2 measurements,
  1 routing-shortcuts og 19 visual-issue. Dette inkluderer Legacy-regresjoner.
- `8a4cf63`: tre målrettede crossing-jumps-tester bestått etter halo-endringen.
  Kryssingsdemoen er også undersøkt visuelt med understrek/bue.
- Fem etterfølgende komplette kjøringer av 11 fokuserte tester bestått.
- Valgrind 3.26 på parallelle/motsattrettede forbindelser og self-loop:
  0 feil, 0 definitely/indirectly lost; 48 bytes possibly lost i Rust-testharness,
  456 bytes reachable. Dette er én avgrenset eierskapstest, ikke full lekkasjeanalyse.
- `cargo fmt --all`, `git diff --check` og SHA-256-kontroll av alle 52
  vendorfiler bestått. Lokal Clippy var ikke installert og er ikke påstått kjørt.

SVG-kryssingspresentasjon og LGPL-metadata er i `8a4cf63`; denne commiten kan
pinnes av XFMD separat fra denne rapport-/artefaktcommiten.

### XFMD-stress og admission

XFMDs faktiske Pango-mål ga traceability 64 ms, service-map 4 ms,
layer-delivery 13 ms og 128-noders kjede 605 ms i en Release-kjøring.
128 noder / 512 tette kanter kunne derimot bli værende lenge i native nudging
uten callback og ble stoppet. Den nye adaptergrensen avviser slike grafer før
native arbeid: maksimalt 256 forbindelser og 2048 kandidatporter. Dette er en
synlig begrensning, ikke en påstand om hard tidsgrense. Egen regresjon kontrollerer
at begge grensene avvises før native transaksjon.

Etter admission-endringen bestod 12 libavoid-tester og 2 måleseam-tester på nytt.
De vanlige før/etter-rutene er uendret; grensen rammer stressinnmatingen.

### Ubuntu og self-loop-regresjon

GitHub Ubuntu avdekket en tvetydig etikett med andre Pango-mål. De faktiske
målene ble hentet i en Ubuntu-container og ga identisk feil lokalt. Et siste
etikett-only-pass på ferdige ruter rettet dette uten å svekke valideringen.
Et sjeldent nullresultat for self-loops blant parallelle kanter ble også funnet;
to eksplisitte, retningsbundne boundary-endepunkter unngår bibliotekets kollaps
av virtuelle ShapeConnectionPin-endepunkter. 50 separate gjentakelser bestod;
regresjonen gjentar nå transaksjonen 32 ganger i selve testen.

Etter rettelsene bestod 12 libavoid- og 3 måleseam-tester, inkludert Ubuntu-data.

Etter `6ff5a54` ble alle seks sammenligninger kjørt på nytt. SVG, rutepunkter
og kvalitetsmål var uendret; bare tidsmålingene ble oppdatert. Ubuntu-fixturen
dekker et annet, eksplisitt målesett enn standardtemaets frosne A/B-eksempler.
