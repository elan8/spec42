# META
~~~ini
description=A flow between features of two ports is an edge between those ports, with the item that flows
type=generate
libraries=standard
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=Feed::links
~~~
# SOURCE
## model.sysml
~~~sysml
package Feed {
    private import StandardViewDefinitions::*;
    item def Fuel;
    port def FuelOut { out item fuel : Fuel; }
    port def FuelIn { in item fuel : Fuel; }
    part def Tank { port outlet : FuelOut; }
    part def Engine { port inlet : FuelIn; port reserve : FuelIn; }
    part vehicle {
        part tank : Tank;
        part spare : Tank;
        part engine : Engine;
        flow of Fuel from tank.outlet.fuel to engine.inlet.fuel;
        flow topUp from spare.outlet to engine.reserve;
    }
    view links : InterconnectionView { expose vehicle; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8b01a17039facd3d330171f591665457112d09d1d9e970d934a2578ed9a0f8d1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelIn")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelIn")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "FuelIn")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~FuelIn") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "FuelOut")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~FuelOut") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelOut")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "tank::outlet::fuel")) (flowTarget (reference "engine::inlet::fuel")) (flowPayloadType (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "spare::outlet")) (flowTarget (reference "engine::reserve")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "tank::outlet::fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "engine::inlet::fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (kind flowSource) (ordinal 0))
      (authored-target "spare::outlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (kind flowTarget) (ordinal 0))
      (authored-target "engine::reserve")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "FuelIn")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "FuelOut")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (named (kind flow) (name "topUp")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 35) (end 6 41)) (probe (position 6 35))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::inlet"))) (kind featureTyping) (ordinal 0) (authored-target "FuelIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 58) (end 6 64)) (probe (position 6 58))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve"))) (kind featureTyping) (ordinal 0) (authored-target "FuelIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 37) (end 4 41)) (probe (position 4 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 39) (end 3 43)) (probe (position 3 39))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 34) (end 5 41)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet"))) (kind featureTyping) (ordinal 0) (authored-target "FuelOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 17) (end 14 36)) (probe (position 14 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 46) (end 14 53)) (probe (position 14 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 26) (end 11 42)) (probe (position 11 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "tank::outlet::fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelOut::fuel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 46) (end 11 63)) (probe (position 11 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "engine::inlet::fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::FuelIn::fuel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 16) (end 11 20)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Fuel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 22) (end 10 28)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 21) (end 9 25)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::spare"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 20) (end 8 24)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::tank"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 24) (end 12 36)) (probe (position 12 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (kind flowSource) (ordinal 0) (authored-target "spare::outlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Tank::outlet")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 40) (end 12 54)) (probe (position 12 40))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::vehicle::topUp"))) (kind flowTarget) (ordinal 0) (authored-target "engine::reserve")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Engine::reserve")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:8b7434be482f6ea7255b2261531ac204c83c2a557cb97403a956abca4b5b384e",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        5,
        25,
        5,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        34,
        5,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        27,
        6,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        35,
        6,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        48,
        6,
        55
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        58,
        6,
        64
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        9,
        7,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        20,
        8,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        21,
        9,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        13,
        10,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        22,
        10,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        64
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        16,
        11,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        26,
        11,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        46,
        11,
        63
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        13,
        12,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        24,
        12,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        40,
        12,
        54
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        9,
        14,
        14
      ]
    }
  ],
  "references": [
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::Engine"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::Engine::inlet"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::Engine::reserve"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::Fuel"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::FuelIn"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::FuelIn::fuel"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::FuelOut"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::FuelOut::fuel"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::Tank"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::Tank::outlet"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::links"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::vehicle"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::vehicle::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::vehicle::engine"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::vehicle::spare"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::vehicle::tank"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Feed::vehicle::topUp"
    },
    {
      "kind": "source-anchor",
      "source": 13,
      "ownerQualifiedName": "Feed::vehicle",
      "metaclass": "FlowUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "typing",
      "ordinal": 38
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 42
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "flow",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "flow",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "typing",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "typing",
      "ordinal": 30
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "flowPayloadType",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "flowSource",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "flowTarget",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "typing",
      "ordinal": 34
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "typing",
      "ordinal": 26
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "typing",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "flowSource",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "flowTarget",
      "ordinal": 19
    }
  ],
  "selectedView": {
    "reference": 10,
    "kind": "interconnection-view",
    "name": "links",
    "source": 20
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "kind": "interconnection-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 11,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "vehicle",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 6,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "direct",
            "members": [
              3,
              5,
              7
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              1,
              2
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 17,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 13,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 16,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "topUp",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 17,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 15,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "tank",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 8,
              "label": "Tank"
            }
          ]
        },
        "owner": 0,
        "source": 7,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              4
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 9,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "outlet",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "FuelOut"
            }
          ]
        },
        "owner": 3,
        "source": 0,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "out"
      },
      {
        "reference": 14,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "spare",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 8,
              "label": "Tank"
            }
          ]
        },
        "owner": 0,
        "source": 9,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              6
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 9,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "outlet",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "FuelOut"
            }
          ]
        },
        "owner": 5,
        "source": 0,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "out"
      },
      {
        "reference": 13,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "engine",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Engine"
            }
          ]
        },
        "owner": 0,
        "source": 11,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              8,
              9
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 1,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "inlet",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 4,
              "label": "FuelIn"
            }
          ]
        },
        "owner": 7,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "in"
      },
      {
        "reference": 2,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "reserve",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 4,
              "label": "FuelIn"
            }
          ]
        },
        "owner": 7,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "in"
      }
    ],
    "relationships": [
      {
        "reference": 29,
        "source": 1,
        "kind": "flowPayloadType",
        "target": {
          "status": "resolved",
          "reference": 3
        },
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 30,
        "source": 1,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "reference": 7
        },
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 31,
        "source": 1,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "reference": 5
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 38,
        "source": 3,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 8
        },
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 22,
        "source": 4,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 39,
        "source": 2,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 6
        },
        "provenance": "authored",
        "navigation": 18
      },
      {
        "reference": 40,
        "source": 2,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 9
        },
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 36,
        "source": 5,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 8
        },
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 23,
        "source": 6,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 34,
        "source": 7,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 18,
        "source": 8,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 4
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 19,
        "source": 9,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 4
        },
        "provenance": "authored",
        "navigation": 5
      }
    ],
    "edges": [
      {
        "reference": 24,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 20,
        "source": 4,
        "target": 8,
        "origin": 1,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 14
      },
      {
        "reference": 25,
        "source": 0,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 37,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 0
      },
      {
        "reference": 26,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 17
      },
      {
        "reference": 21,
        "source": 6,
        "target": 9,
        "origin": 2,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 18
      },
      {
        "reference": 27,
        "source": 0,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 35,
        "source": 5,
        "target": 6,
        "origin": 6,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 0
      },
      {
        "reference": 28,
        "source": 0,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 32,
        "source": 7,
        "target": 8,
        "origin": 8,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 2
      },
      {
        "reference": 33,
        "source": 7,
        "target": 9,
        "origin": 9,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 4
      }
    ],
    "metadata": {
      "parts": [
        0,
        3,
        5,
        7
      ],
      "ports": [
        4,
        6,
        8,
        9
      ],
      "connectors": []
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
