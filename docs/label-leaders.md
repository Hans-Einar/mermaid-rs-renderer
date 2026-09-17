# Etikettpekere i SVG

`render::add_label_leaders(svg, layout, theme, config)` er et eksplisitt,
valgfritt presentasjonssteg for flowcharts. Layout/rutepunkter er immutable.
Retur er SVG og antall utelatte pekere. XFMD aktiverer steget etter eksisterende
SVG-rendering, også når kryssingshopp brukes.

Søk begge sider av etiketten. Frie venstrekandidater prioriteres; innen siden
velges korteste, med segmentindeks som stabil tie-break. Loddrett mål gir
vannrett strek fra sidens midtpunkt. Vannrett mål gir 45° ut fra øvre/nedre
hjørne og så loddrett til segmentet. Diagonalbenet er maks 8 enheter og høyst
halve gapet. Miter/butt, bredde 1, prikkradius 2,2; vanlig kantbredde er 2.
Fargen følger theme.line_color og kan styles gjennom SVG.

Geometrien bruker faktisk tekstboks og samme padding som SVG-rendereren.
Treffpunktet ligger minst 8 enheter fra segmentender. Pekeren unngår node- og
tittelbokser, andre etiketter, kanter og tidligere pekere. Prikken holder
avstand til andre segmenter, blant annet for å unngå kryssingshopp. Alle
kandidater er maksimalt 100 enheter lange. Over 256 kanter / 4096 rutepunkter
utelates steget med diagnostikk. Ingen fri kandidat: behold diagram/etikett,
utelat peker; ingen tvungen kryssing og ingen ny ruting.

Tester dekker venstreprioritet, høyrefallback, blokkering, skarp 45°-geometri,
diagonale kryssinger, prikk og uendrede logiske ruter. Kompakte diagrammer kan
mangle noen pekere. Ingen garanti om peker for hver etikett når plassen er knapp.
