---
document_id: SDP-12-02
profile: sdp-system-development-pilot/0.1
status: draft
updated: 2026-09-18
---

# Recap — klar for designgjennomgang før parallell utvikling

## Hva som er konkretisert

XFMD er den påkrevde interaktive verten for BoxUI i Markdown. Ingen HTML-side
eller nettleser er leveransen. Forken eier widgets/modell/layout/SVG; XFMD eier
native FOX-input, fokus, sessions, bindinger og dispatch. Domene/simulering har
egen port og overtar ikke rendererens ansvar.

[Kontrakten](../06--Container-Design/06-02--XFMD-Host-Contract.md) beskriver kilde,
normalisert modell, frame/control map, native redigering, gamle hendelser,
parallelle forberedelser, eierskap/FFI, grenser og PDF. Barnediagrammer tolkes før
layout og kommer inn som forberedte SVG/error-verdier. Dette ble presisert under review.

Felles draft `BX-HOST/0.1-draft1` finnes i designcommit `54aee4e`.
Seks skjemaer og positive/negative fixtures gir et konkret grensesnitt begge
arbeidsspor kan bruke. Mock-rammer er uttrykkelig ikke renderbevis.
[SDL-modellen](../07--Detailed-Design/SDL/BoxUi.design) valideres med dagens
strukturelle design-core 0.1. Atferd/guards/widgettyper er fortsatt separate
kontrakter, siden dagens parser ikke kan uttrykke eller kjøre disse.

## Egen XFMD-oppgave

Repository: Hans-Einar/xfmd. Worktree: `/home/warloc/git/xfmd-boxui`.
Branch: `sprint/003/phase/044-boxui-design`. Registreringscommit: `69e74886ae854c6bdc0fcb2699e266597fe44d11`.
Oppgave: `docs/design/boxui-integration.md`; Sprint 003 / fase 044;
UR-043 / SR-026 / FTR-011 (Proposed/Future).

Renderer R1–R3 og XFMD X1–X3 kan utvikles mot hverandres kontraktfixtures etter
review. R4/X4 er faktisk integrasjon med eksakte pins og GUI/PDF-bevis.
[Arbeidspakker og gate](../08--Realization/08-02--Delivery-Plan.md).
XFMD-implementasjonen er ikke startet; begge hovedbrancher og installasjonen er uendret.

## Fire valg til neste iterasjon

1. **Kildeform:** anbefalt strict JSON etter `boxui 0.1` i første profil.
   Avgjør om lesbar kortsyntaks er viktig nok til å prioritere allerede nå.
2. **Widgetomfang:** anbefalt text/value/button/enkeltlinjet input/diagram,
   row/column. Tabs, collapse og grid fra Concept1 kommer senere.
3. **Kjøring og eksport:** anbefalt eksplisitt lokal syntetisk modus; PDF viser
   aksepterte verdier, ikke usendte utkast. TCP og scenario-DSL er senere arbeid.
4. **Native grenser:** anbefalt FOX-felt med Unicode/clipboard og definerte
   fokusregler. Faktisk IME/tilgjengelighet, clipping og budsjetter må måles i
   første host-spike før noen kan love full plattformstøtte.

Dette er reviewvalg med foreslåtte svar, ikke skjulte programmeringsoppgaver.
Endret beslutning reviderer kontrakten og begge oppgavene før parallellstart.

## Bevis og begrensning

[Kontrollbevis](../09--Verification/09-02--Evidence.md): lokale manifest-/skjema-/
referansekontroller, godkjent strukturell SDL og avvist reversert eierskap.
XFMDs blueprint- og symbolkontroller består. Ingen BoxUI-produktkode, GUI/PDF-test,
ny rendering, programbygg eller installasjon er gjort i denne designleveransen.

**Stoppunkt nå: G-PARALLEL-REVIEW.** Ta recap og designiterasjonen før oppstart.
