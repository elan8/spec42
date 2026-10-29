# META
~~~ini
description=Flows between the features of two connected ports run along their connector, one each way
type=generate
libraries=standard
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=Dispatch::links
~~~
# SOURCE
## model.sysml
~~~sysml
package Dispatch {
    private import StandardViewDefinitions::*;
    item def Assignment;
    item def Status;
    port def DispatchPort { out assignment : Assignment; in status : Status; }
    part def Group { port car : DispatchPort; }
    part def Car { port group : ~DispatchPort; }
    part service {
        part group : Group;
        part car : Car;
        connect group.car to car.group;
        flow group.car.assignment to car.group.assignment;
        flow car.group.status to group.car.status;
    }
    view links : InterconnectionView { expose service; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b0846ff1b654c9281e2511d8150b2207ab04379c180d87d55d5d90ecc76e6009") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DispatchPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind port-def) (name "DispatchPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~DispatchPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Assignment") (direction out)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Status") (direction in)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DispatchPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "service")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "group::car")) (memberAccessOperand (reference "car::group")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "group::car::assignment")) (flowTarget (reference "car::group::assignment")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "car::group::status")) (flowTarget (reference "group::car::status")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Car")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Group")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (kind featureTyping) (ordinal 0))
      (authored-target "DispatchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (kind featureTyping) (ordinal 0))
      (authored-target "Assignment")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (kind featureTyping) (ordinal 0))
      (authored-target "DispatchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "service")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "group::car")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "car::group")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "group::car::assignment")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0))
      (authored-target "car::group::status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "car::group::assignment")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0))
      (authored-target "group::car::status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (kind featureTyping) (ordinal 0))
      (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (kind featureTyping) (ordinal 0))
      (authored-target "Group")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind port-def) (name "DispatchPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind port-def) (name "DispatchPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind port-def) (name "DispatchPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind port-def) (name "DispatchPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind port-def) (name "DispatchPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind port-def) (name "DispatchPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1)) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group")) (scopes any))
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
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group")) (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) "group::car")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car")) (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) "car::group")))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 33) (end 6 45)) (probe (position 6 33))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group"))) (kind featureTyping) (ordinal 0) (authored-target "DispatchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 45) (end 4 55)) (probe (position 4 45))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment"))) (kind featureTyping) (ordinal 0) (authored-target "Assignment")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Assignment")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 69) (end 4 75)) (probe (position 4 69))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status"))) (kind featureTyping) (ordinal 0) (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Status")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 32) (end 5 44)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car"))) (kind featureTyping) (ordinal 0) (authored-target "DispatchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 17) (end 14 36)) (probe (position 14 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::links"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 46) (end 14 53)) (probe (position 14 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "service")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 16) (end 10 25)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "group::car")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group::car")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 29) (end 10 38)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "car::group")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car::group")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 13) (end 11 33)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "group::car::assignment")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 13) (end 12 29)) (probe (position 12 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0) (authored-target "car::group::status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 37) (end 11 57)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "car::group::assignment")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::assignment")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 33) (end 12 49)) (probe (position 12 33))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Dispatch")) (named (kind part) (name "service")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0) (authored-target "group::car::status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::DispatchPort::status")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 19) (end 9 22)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::car"))) (kind featureTyping) (ordinal 0) (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Car")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 21) (end 8 26)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::service::group"))) (kind featureTyping) (ordinal 0) (authored-target "Group")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Dispatch::Group")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:00a972be53525f1b9b05386657c7bad5be403a84812cd70335a690941ef1d506",
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
        26,
        5,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        32,
        5,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        24,
        6,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        33,
        6,
        45
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
        18
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        21,
        8,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        19,
        9,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        8,
        10,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        16,
        10,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        29,
        10,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        58
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        13,
        11,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        37,
        11,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        8,
        12,
        50
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        13,
        12,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        33,
        12,
        49
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
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::Car"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::Car::group"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::DispatchPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::DispatchPort::assignment"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::DispatchPort::status"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::Group"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::Group::car"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::links"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::service"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::service::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::service::car"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Dispatch::service::group"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Dispatch::service",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowUsage",
      "ownerQualifiedName": "Dispatch::service",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowUsage",
      "ownerQualifiedName": "Dispatch::service",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "flow",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "connection",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "flow",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "flowSource",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "flowSource",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "flowTarget",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "flowTarget",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "memberAccessOperand",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "memberAccessOperand",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "noncomposite-membership",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 11
    }
  ],
  "selectedView": {
    "reference": 7,
    "kind": "interconnection-view",
    "name": "links",
    "source": 18
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 6,
        "provenance": "authored",
        "reference": 20,
        "source": 0,
        "target": 6
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 7,
        "provenance": "implied",
        "reference": 33,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 1,
        "provenance": "authored",
        "reference": 21,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 10,
        "origin": 1,
        "provenance": "authored",
        "reference": 17,
        "source": 7,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 2,
        "provenance": "authored",
        "reference": 22,
        "source": 0,
        "target": 2
      },
      {
        "kind": "flow",
        "navigation": 13,
        "origin": 2,
        "provenance": "implied",
        "reference": 18,
        "source": 7,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 3,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 3
      },
      {
        "kind": "flow",
        "navigation": 16,
        "origin": 3,
        "provenance": "implied",
        "reference": 15,
        "source": 5,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 4,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 4
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 5,
        "provenance": "implied",
        "reference": 31,
        "source": 4,
        "target": 5
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [
        1
      ],
      "parts": [
        0,
        4,
        6
      ],
      "portFamilies": [
        {
          "connectors": [
            1
          ],
          "general": [
            {
              "label": "DispatchPort",
              "reference": 2
            }
          ],
          "types": [
            {
              "label": "DispatchPort",
              "reference": 2
            }
          ]
        }
      ],
      "ports": [
        5,
        7
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              4,
              6
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1,
              2,
              3
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "service",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:7564fd3c6de36d869def92320659dd00344ca8337d88bb7ffd821cc11ab5d814",
        "reference": 8,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:4d9fc51b8291b104d60cb05fb51f28d0840e2e8f1ff1e75fa29df5acae1d4ba1",
        "reference": 12,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:7d549a16d484212dd6ee3b3206f80b016776cd4a720edf7ad91ff5d687cea909",
        "reference": 13,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:57e4732855fde790acd4d76c1bddac17b3afd21ab5a0cb86abd5b46ab0dcd1e5",
        "reference": 14,
        "source": 15,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              5
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "car",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a6c770f8e6029ca4bec0cff108366052b6e6125067eb8907ae1bc0af5b849437",
        "reference": 10,
        "source": 7,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Car",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "inout",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "group",
        "notationRole": "usage",
        "owner": 4,
        "presentationKey": "p:b41d0318c7574b4845d4875cc2456a5349b985581ff3900f2d53a704bb205969",
        "reference": 1,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DispatchPort",
              "reference": 2
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              7
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "group",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:fa8caca1f7cbb6d713c5a5bdc7f5b5efe33cb67f358502ad3932e5a5c73da222",
        "reference": 11,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Group",
              "reference": 5
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "inout",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "car",
        "notationRole": "usage",
        "owner": 6,
        "presentationKey": "p:ed38446bbd1101a4614d7f4aab1401d2ff0a98e37fb3b2cc884447a4e129ddc1",
        "reference": 6,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DispatchPort",
              "reference": 2
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 34,
        "source": 6,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 19,
        "source": 7,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 10,
        "provenance": "authored",
        "reference": 29,
        "source": 1,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 11,
        "provenance": "authored",
        "reference": 30,
        "source": 1,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 13,
        "provenance": "authored",
        "reference": 25,
        "source": 2,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 14,
        "provenance": "authored",
        "reference": 27,
        "source": 2,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 16,
        "provenance": "authored",
        "reference": 26,
        "source": 3,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 17,
        "provenance": "authored",
        "reference": 28,
        "source": 3,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 32,
        "source": 4,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 16,
        "source": 5,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
