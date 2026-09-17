# Beskyttet etikettilhørighet

Kantens tekstboks knyttes til nærmeste ortogonale segment med overlappende
projeksjon. Det åpne rektangelet mellom segmentet og tekstboksen er en
beskyttet tilhørighetssone: ingen fremmed kant får krysse den. Kant-ID er
indeksen, også for parallelle kanter. Felles endepunkter andre steder berøres ikke.

Plassering prioriterer kandidater uten soneinntrenging. Ved nødvendig ny ruting
utvides teksthindringen mot eiersegmentet, men stopper nøyaktig én hindringsmargin fra segmentet. Hindringer er globale i libavoid;
eierkanten får ingen særbehandling. Sonevalidering på ferdige ruter er derfor
nødvendig også etter omruting og siste etikettpass. Ingen nye ubegrensede pass.
Uløselig tilhørighet gir NoSpace, ikke skjult overlapp eller Legacy-fallback.

## Senere portsøk (ikke implementert)

Adapteren mottar plasserte former og forbindelser med kilde/mål, ikke ferdige
linjer. Første pass velger blant porter; senere pass bruker kantspesifikke
låste portklasser og de faktisk valgte koordinatene. Dette låser tilordningen,
men er ikke en global fysisk portreservasjon: ulike pin-objekter kan ha samme
koordinat. Et portsøk må kontrollere fysisk belegg og klaring eksplisitt.

Prøv et lite deterministisk kandidatsett og rut alle berørte forbindelser på
nytt. Godta bare streng forbedring av samlet kostnad etter alle valideringer;
bruk besøkte portkonfigurasjoner og et fast tids-/forsøksbudsjett. Geometrisk
nærmeste porter garanterer ikke korteste hindringsfrie rute. Stopp ved lokal
stabilitet eller budsjett, uten å hevde globalt optimum.

Et ekstra toleransegap ble prøvd og forkastet: native ruting kunne legge en
fremmed linje i den smale åpningen. Den globale bufferen skal derfor nå helt
fram til eiersegmentet. Sammenfallende linjer på sonegrensen er fortsatt et
separat parallellavstandsproblem, synlig i kvalitetsdiagnostikken.

Etikettplassering kan prøve fire faste rekkefølger (areal synkende/stigende,
kant-ID stigende/synkende). Den første fullt validerte plasseringen beholdes;
ellers brukes første kandidat til neste eksisterende rutepass. Dette er
avgrenset til samme opprinnelige arbeidsbudsjett.
