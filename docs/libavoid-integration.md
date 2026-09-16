# Utskiftbar rutemotor

## Beslutning og base

Arbeidet bygger på forkens `f64ab1f`, med upstream `3726ccb`. Tidligere forsøk
bevares som Legacy. Upstream/master er kontrollert og står på samme base.
`Current`, `Dagre`, `Auto` er nodeplassering, ikke rutemotorer.

Det nye Rust-grensesnittet mottar stabile node-ID-er, separate kant-ID-er,
polygoner, porter/retninger, etiketthindringer og arbeidsbudsjett. Resultatet
eier ruter og valgte endepunkter. Ingen libavoid-typer krysser adapteren.
Ny layoutinngang returnerer Result og diagnostikk; ingen stille Legacy-fallback.
Legacy beholdes eksplisitt, og sammenligning på frosset Layout støttes.

## Faser

1. Kontrakt, reproduksjonsgrunnlag og integrasjonspunkt.
2. Låst libavoid og C-ABI med kooperativt avbrudd, samlet transaksjon.
3. Formtilpassede porter, avgrenset etikettplassering og validering.
4. Valgfrie SVG-kryssingshopp etter ferdig ruting.
5. Separat SVG-omlegging i XFMD, deretter konkret fork-pin og rutervalg,
   bare etter dokumentert forbedring. Ingen rutepatcher i XFMD/.deps.

## Bibliotek og bygg

Libavoid er låst til `840ebcff20dbba36ad03a2160edf7cbaf9859984`.
Uendrede C++-kilder og headers ligger i vendor/libavoid; SHA-256-manifest i
vendor/libavoid.lock.json. Bygg laster ikke ned bibliotekkode. C++17-kompilator,
arkiveringsverktøy og Rust/Cargo trengs; valgfri Cargo-feature `libavoid`.

Crates.io-søk ga ingen passende libavoid-binding i dette miljøet. En liten
C-ABI velges for eksplisitt eierskap og liten integrasjonsflate. Routeren eier
shapes/pins/connectors; ruter kopieres til Rust-eide verdier gjennom en synkron callback. Ingen native
resultatpekere overlever kallet.
Exceptions fanges i C++; callbacks får ikke unwinde over grensen. Én lokal
router per kall, ingen delt muterbar router eller global livsløpstilstand.

Libavoid har LGPL-2.1-or-later; forkens øvrige kode forblir MIT. Lisens ligger
ved kildene. Ved statisk distribusjon må mottaker kunne relinke applikasjonen
med endret libavoid: lever relevante objektfiler/arkiver, byggoppskrift og
korresponderende LGPL-kilde/lisenstekst, eller bruk en egnet delt bibliotek-
distribusjon. Å bare legge ved MIT-lisensen er ikke tilstrekkelig.

## Autoritativ rute

Ny backend kjøres uten Legacy-portvalg, omruting og reparasjonskjede.
Nodeplassering og tekstmåling beholdes. Avsluttende koordinattranslasjon er
kompatibel; Legacy-etikettpass og aspektfolding etter ruting er ikke det.
Etiketter plasseres fra faktiske mål og de nye rutene. Et begrenset antall
transaksjoner kan reservere etikettbokser som hindringer. Uløselig plassmangel
skal gi feil/diagnostikk; tekst skal ikke skjules.

Grupper behandles som semantiske medlemsgrenser, ikke ugjennomtrengelige
hindringer for forbindelser til medlemmer. Gruppetittel er teksthindring.
Kryssingsstraff starter på null; svingkostnaden er endelig. Nudging eies av
libavoid og må kontrolleres sammen med pin-valg.

## Avbrudd

Router::shouldContinueTransactionWithProgress gir kooperativt avbrudd.
Libavoid kan bruke tid på å avslutte en fase/rydde opp etter avbrudd. Inputgrenser
og kontroller før/etter transaksjonen supplerer callbacken; dette er ingen hard
sanntidsfrist. Delresultater etter avbrudd publiseres ikke som vellykket ruting.

## Kilder

- https://www.adaptagrams.org/documentation/classAvoid_1_1Router.html
- https://www.adaptagrams.org/documentation/namespaceAvoid.html
- https://www.adaptagrams.org/documentation/classAvoid_1_1ShapeConnectionPin.html
- https://users.monash.edu/~mwybrow/papers/wybrow-gd-2009.pdf
- https://users.monash.edu/~mwybrow/papers/marriott-diagrams-2014.pdf

## Implementert milepæl 2

`layout::routed::compute` velger ruter separat fra `LayoutConfig` sin plassering.
`route_positioned` erstatter bare ruter/etikettankre i en klonet Layout. Legacy
kjører fortsatt gammel kode. Libavoid kjøres etter nodeplassering og før kun
uniform slutttranslasjon/bounds. Ingen Legacy-reparasjon følger etterpå.

Portklasser gir libavoid flere tillatte tilkoblingspunkter. Eksklusive porter
og nudging av mellomsegmenter skiller forbindelser. Sluttsegment-nudging brukes i første pass når alle tilkoblede noder er
rektangler og kontrakten tillater glidning langs sidene. Faktiske porter beholdes
ved senere etikettpass. For øvrige former er sluttsegment-nudging deaktivert:
biblioteket kan ellers flytte endepunkter bort fra den synlige formen. Self-loops bruker
forskjellige sider (høyre/bunn), slik at nullruter ikke blir valgt.

Etikettplassering reserverer målte bokser med klaring; maksimalt tre samlede
rutetransaksjoner brukes. Valgte porter låses mellom passene. Kollisjon eller
frakoblet etikett gir `NoSpace`, aldri skjult tekst. Ingen automatisk fallback.
Gruppeendepunkter og endpoint labels er foreløpig eksplisitt avvist; forbindelser
mellom gruppemedlemmer og omverdenen støttes.

Verifikasjon: `cargo test --locked --no-default-features --features libavoid
--test libavoid_suite` (6 tester), samt `python3 tools/verify_libavoid.py` (52
kilder). Målinger og videre regresjonsdekning følger i neste milepæl.

## Porter, validering og sammenligning (milepæl 3)

Ti integrasjonstester dekker hindringer, smal korridor, avbrudd i callback,
ugyldig input, parallelle/motsattrettede kanter, self-loops, fan-in/out, grupper,
XFMDs fire former, alle tre plasseringsmotorer og den obligatoriske grafen.
Frosne nodeposisjoner og tekstmål sammenlignes eksplisitt; ruter og etikettankre
må gjentas deterministisk. Analytiske formtester supplerer rutelengde, svinger,
kryssinger, parallelle overlapp/nærføringer og etikettkollisjoner i
`layout::routed::quality::measure`. Kryssinger er ikke en hard feil.

Rektangulære nodeomslag brukes konservativt som hindringer; porter ligger på
den synlige formen. Runde hjørner behandles konservativt ved validering.
Tilgjengelige porter er endelige (3–15 per side). Overbelastning av porter eller
utilstrekkelig plass kan derfor gi eksplisitt feil. Nudging har en ønsket avstand,
ikke en garanti i trange korridorer; resterende nærføringer rapporteres.

## Kryssingshopp (milepæl 4)

`render::render_svg_with_crossings` er opt-in. Høyere kantindeks får buen ved
strengt indre, ortogonale kryssinger. Felles endepunkter, kollineære overlapp,
korte avstander til hjørner/pilspisser, noder, tekst, tredje segment og andre
hopp undertrykkes konservativt. Logiske punkter endres ikke. Radius begrenses
til 1–8; diagrammer med over 4096 rutepunkter får vanlig SVG uten hopp for å
begrense det kvadratiske presentasjonssøket. Tre tester dekker kryssingsregler,
prioritet, klaring og at ruten forblir uendret.

## Eksterne tekstmål og XFMD-seam

XFMDs eksisterende måle-/fristpatch er flyttet hit med kontekstbasert treveis-
integrasjon mot den undersøkte upstream-basen. Forkens tidligere Legacy-
forbedringer er bevart. `measurements::with_measurements` gir begge strategier
samme eksterne TextBlock-verdier og scoped, trådlokal deadline. Tilstand gjenopprettes
også ved nesting og unwind. Native libavoid-avbrudd returnerer Result; gamle
Rust-checkpoints kan fortsatt unwinde til XFMDs etablerte panic-barriere.
To egne tester dekker faktisk måleverdi i ny ruter og TLS-opprydding.
