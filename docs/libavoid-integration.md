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
shapes/pins/connectors; resultatet eies separat og frigjøres via samme ABI.
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
