# Verbeterpunten voor `elk-rust` in `sysml-language-server`

Dit document vat samen welke verbeteringen in `elk-rust` de meeste impact hebben op de diagramkwaliteit in `sysml-language-server`.

## Prioriteiten

### 1. Betere cross-hierarchy routing

Verbindingen tussen nested onderdelen en nodes buiten dezelfde container zijn nog een zwak punt. Vooral `general-view` en complexe `interconnection-view` fixtures winnen veel als edges compound boundaries slimmer verlaten en weer binnenkomen.

### 2. Echte edge label placement

Labels zitten nu functioneel nog grotendeels in onze SVG-laag. Als `elk-rust` labels zelf goed kan plaatsen zonder overlap met nodes, ports en andere labels, wordt de output stabieler en hoeven we minder workarounds te houden.

### 3. Sterkere handling van self-loops en parallel edges

Self-loops lopen nu nog via fallback-routes. Ook parallelle connectors mogen duidelijker gescheiden worden, met consistente lanes of bundling.

### 4. Meer controle over ordering en layer constraints in compound graphs

Voor `general-view` gebruiken we nog synthetische structuur en model-order hints. `elk-rust` zou beter moeten omgaan met:

- sibling ordering binnen containers
- vaste “eerste/laatste” plaatsing
- component packing van meerdere root-blokken

### 5. Betere port semantics

Voor `interconnection-view` is dit belangrijk:

- `fixed order` echt respecteren in drukke diagrammen
- betere spacing tussen ports en labels
- voorspelbare side- en anchor-keuze bij veel connectors op dezelfde node

### 6. Routing quality metrics en regressiegates in de engine zelf

`elk-rust` heeft al goede rapportage, maar voor SysML zijn vooral deze metrics nuttig:

- `edge-node intrusion count`
- `crossing count` per subgraph
- bend budget
- label overlap stats
- per-phase quality deltas

### 7. Minder extreme canvasgroei bij grote SysML fixtures

Bij dense graphs kan de layout nog te breed of te hoog uitvallen. Compactere packing zonder leesbaarheid te verliezen zou de output meteen verbeteren.

### 8. SysML-specifieke tuning als first-class profile

De bestaande `GeneralView` en `InterconnectionView` profielen zijn een goede start. Wat nog helpt:

- betere defaults voor compound part trees
- betere defaults voor orthogonal bus-achtige connectoren
- voorkeur voor “structure first, relations second” layouts

## Samenvatting

De grootste winst voor `sysml-language-server` zit in:

1. compound-aware routing
2. edge label placement
3. port- en connector-kwaliteit

Als deze drie gebieden verbeteren, worden zowel `general-view` als `interconnection-view` merkbaar leesbaarder en stabieler.
