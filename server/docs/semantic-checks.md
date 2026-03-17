# Semantische checks in Spec42

Dit document beschrijft de semantische validaties die we uitvoeren naast de syntax/parse-checks, en een backlog van checks die later toegevoegd kunnen worden.

## Huidige checks (geïmplementeerd)

Deze checks draaien in `server/src/semantic_checks.rs` zodra het document succesvol parsed is en in de semantic graph zit. Ze worden gepubliceerd als LSP-diagnostics samen met eventuele parsefouten.

| Check | Severity | Code | Beschrijving |
|-------|----------|------|--------------|
| **Connection endpoint niet een port** | Warning | `connection_endpoint_not_port` | Een `connect A to B` waar A of B geen port is (bijv. part of attribute). |
| **Port type mismatch** | Warning | `port_type_mismatch` | Twee verbonden ports hebben geen compatibel type: in SysML moet hetzelfde base type zijn, met één geconjugeerd (`~T`) en één niet (`T`). |
| **Unconnected port** | Information | `unconnected_port` | Port die nergens aan verbonden is. |
| **Duplicate connection** | Information | `duplicate_connection` | Dezelfde twee endpoints meer dan eens verbonden. |

---

## Backlog: checks om later toe te voegen

### Connecties & poorten

- **Unresolved connection**  
  Bij `connect X to Y`: als X of Y niet in de graph als node voorkomt (niet kon worden opgelost), een diagnostic “endpoint X (of Y) could not be resolved”. Vereist dat de graph builder bijhoudt welke connect-statements geen edge hebben gekregen.

- **Port zonder type**  
  Information/warning voor ports zonder `portType`, omdat type-compatibility dan niet te checken is.

- **Direction compatibility (ports)**  
  Als `direction` (in/out) op ports wordt gebruikt: controleren dat verbonden ports logisch matchen (bijv. out → in), eventueel als info/warning.

### Typing & specialisatie

- **Onbekend type (part/port/attribute/requirement/action)**  
  Part/port/attribute met `partType` / `portType` / `attributeType` waar die type-reference nergens als part def / port def / interface gedefinieerd is. Idem voor requirement/use case/action met een type dat niet bestaat.

- **Cycles in typing/specializes**  
  Waarschuwing bij circulaire typing of specialisatie (bijv. A : B, B : A of via een langere keten).

### Overige relaties

- **Bind / allocate / satisfy**  
  Soortgelijke checks als bij connections: eindpunten bestaan, zijn van het juiste element type, en (waar van toepassing) type-compatibel.

### Multiplicity & waarden

- **Multiplicity**  
  Waar multiplicity wordt gebruikt: controleren dat waarden geldig zijn (bijv. `0..*`, `1..1`) en eventueel dat verbindingen met multiplicity kloppen.

---

## Technische aantekeningen

- Semantische diagnostics worden alleen berekend als **geen parsefouten** zijn (`result.errors.is_empty()`).
- De graph wordt per document bijgewerkt bij `didOpen` / `didChange`; daarna wordt `publish_diagnostics_for_document` aangeroepen en daarin `semantic_checks::compute_semantic_diagnostics(&state.semantic_graph, &uri_norm)`.
- Connection-edges worden opgehaald via `SemanticGraph::connection_edge_node_pairs_for_uri(uri)`; port-kinds via `ibd::is_port_like(kind)`.
