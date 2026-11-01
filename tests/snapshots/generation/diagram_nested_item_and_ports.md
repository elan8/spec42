# META
~~~ini
description=Interconnection presentation retains item ownership of nested parts and resolved identities of nested ports used by bindings (SysML 8.2.3.10-12)
type=generate
libraries=standard
require_complete_publication=true
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=NestedOwners::selected
~~~
# SOURCE
## model.sysml
~~~sysml
package NestedOwners {
    private import StandardViewDefinitions::*;
    part def Slot;
    item def Payload;
    item def Box { part slot : Slot; }
    port def BoxPort { inout item box : Box; }
    port def P { out item payload : Payload; }
    port def Bundle { port a : P; port b : P; }
    part def Source { port out1 : P; }
    part def Destination { port inBox : BoxPort; ref item carried : Box; }
    part def Assembly {
        port bundle : Bundle;
        port second : Bundle;
        part src : Source;
        part destination : Destination;
        bind bundle.a = src.out1;
    }
    view selected : InterconnectionView { expose Assembly; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1cbfc112f1c55987035e120fcfd6af0014525834ea4643ce59ee5acf64e28c69") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "bundle::a")) (memberAccessOperand (reference "src::out1")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bundle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Destination")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bundle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Source")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Slot")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind port-def) (name "BoxPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~BoxPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind port-def) (name "Bundle")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Bundle") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BoxPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~P") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Payload")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Assembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "bundle::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "src::out1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bundle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (kind featureTyping) (ordinal 0))
      (authored-target "Destination")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bundle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (kind featureTyping) (ordinal 0))
      (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Slot")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box"))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried"))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (kind featureTyping) (ordinal 0))
      (authored-target "BoxPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload"))) (kind featureTyping) (ordinal 0))
      (authored-target "Payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::subparts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind port-def) (name "BoxPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind port-def) (name "Bundle")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port::subports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port::subports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::subitems"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::subparts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::subitems")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::subparts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port::subports"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port::subports")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port::subports"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port::subports")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 13) (end 15 21)) (probe (position 15 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "bundle::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 24) (end 15 32)) (probe (position 15 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind part-def) (name "Assembly")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "src::out1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 22) (end 11 28)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::bundle"))) (kind featureTyping) (ordinal 0) (authored-target "Bundle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 27) (end 14 38)) (probe (position 14 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::destination"))) (kind featureTyping) (ordinal 0) (authored-target "Destination")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 22) (end 12 28)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::second"))) (kind featureTyping) (ordinal 0) (authored-target "Bundle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 19) (end 13 25)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly::src"))) (kind featureTyping) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 31) (end 4 35)) (probe (position 4 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box::slot"))) (kind featureTyping) (ordinal 0) (authored-target "Slot")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Slot")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 40) (end 5 43)) (probe (position 5 40))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort::box"))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 31) (end 7 32)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::a"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 43) (end 7 44)) (probe (position 7 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Bundle::b"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 68) (end 9 71)) (probe (position 9 68))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::carried"))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Box")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 40) (end 9 47)) (probe (position 9 40))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Destination::inBox"))) (kind featureTyping) (ordinal 0) (authored-target "BoxPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::BoxPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 36) (end 6 43)) (probe (position 6 36))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P::payload"))) (kind featureTyping) (ordinal 0) (authored-target "Payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Payload")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 34) (end 8 35)) (probe (position 8 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Source::out1"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::P")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 20) (end 17 39)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 49) (end 17 57)) (probe (position 17 49))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NestedOwners")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NestedOwners::Assembly")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:4451113027424aa32952b26966534f20f41cb0290f42cf2ae08c9f172e27c0ee",
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
        4,
        24,
        4,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        31,
        4,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        34,
        5,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        40,
        5,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        27,
        7,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        31,
        7,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        39,
        7,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        43,
        7,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        27,
        8,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        34,
        8,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        32,
        9,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        40,
        9,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        58,
        9,
        65
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        68,
        9,
        71
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        13,
        10,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        13,
        11,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        22,
        11,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        13,
        12,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        22,
        12,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        13,
        13,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        19,
        13,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        13,
        14,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        27,
        14,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        8,
        15,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        13,
        15,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        24,
        15,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        9,
        17,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Assembly"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Assembly::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Assembly::bundle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Assembly::destination"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Assembly::second"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Assembly::src"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Box"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Box::slot"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::BoxPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::BoxPort::box"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Bundle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Bundle::a"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Bundle::b"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Destination"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Destination::carried"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Destination::inBox"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::P"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Slot"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Source"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::Source::out1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NestedOwners::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "BindingConnectorAsUsage",
      "ownerQualifiedName": "NestedOwners::Assembly",
      "source": 23,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "noncomposite-membership",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "noncomposite-membership",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "noncomposite-membership",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "memberAccessOperand",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "memberAccessOperand",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "noncomposite-membership",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "noncomposite-membership",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "noncomposite-membership",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "typing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "connection",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 19
    }
  ],
  "selectedView": {
    "reference": 20,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 26
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "noncomposite-membership",
        "navigation": 23,
        "origin": 1,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 24,
        "origin": 1,
        "provenance": "authored",
        "reference": 44,
        "source": 11,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 8,
        "provenance": "authored",
        "reference": 22,
        "source": 0,
        "target": 8
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 9,
        "provenance": "implied",
        "reference": 38,
        "source": 8,
        "target": 9
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 15,
        "origin": 10,
        "provenance": "authored",
        "reference": 25,
        "source": 0,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 11,
        "provenance": "implied",
        "reference": 29,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 12,
        "provenance": "implied",
        "reference": 30,
        "source": 10,
        "target": 12
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 17,
        "origin": 13,
        "provenance": "authored",
        "reference": 26,
        "source": 0,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 14,
        "provenance": "implied",
        "reference": 35,
        "source": 13,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 15,
        "provenance": "implied",
        "reference": 36,
        "source": 13,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 21,
        "origin": 2,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 2
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 5,
        "provenance": "implied",
        "reference": 32,
        "source": 2,
        "target": 5
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 6,
        "provenance": "implied",
        "reference": 51,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 7,
        "provenance": "implied",
        "reference": 42,
        "source": 6,
        "target": 7
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 12,
        "origin": 3,
        "provenance": "implied",
        "reference": 33,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 4,
        "provenance": "implied",
        "reference": 49,
        "source": 3,
        "target": 4
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [],
      "parts": [
        0,
        2,
        4,
        7,
        8
      ],
      "portFamilies": [
        {
          "connectors": [
            1
          ],
          "general": [
            {
              "label": "P",
              "reference": 16
            }
          ],
          "types": [
            {
              "label": "P",
              "reference": 16
            }
          ]
        }
      ],
      "ports": [
        5,
        9,
        10,
        11,
        12,
        13,
        14,
        15
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              2,
              8
            ],
            "provenance": "direct"
          },
          {
            "kind": "ports",
            "members": [
              10,
              13
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartDefinition",
        "multiplicity": null,
        "name": "Assembly",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:970a82bf806917e08758832de2c67717fca95bde4db5266c8eb5f0374152984a",
        "reference": 0,
        "source": 14,
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
        "metaclass": "BindingConnectorAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:62167542951b3e0afa8da5957ddf78edd5a904da4876cff37c1deddc719d9ba0",
        "reference": 21,
        "source": 23,
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
          },
          {
            "kind": "items",
            "members": [
              3
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "destination",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:38c29e606c4299e0fbd47775f95c789ca51f6ac1505d24ef2b0b3225de65fb29",
        "reference": 3,
        "source": 21,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Destination",
              "reference": 13
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              4
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ItemUsage",
        "multiplicity": null,
        "name": "carried",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:8e5ad0fe7c61e04d228ffdca86355aedf53cd05ef9a24ebd782053d7d378a9b4",
        "reference": 14,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Box",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "slot",
        "notationRole": "usage",
        "owner": 3,
        "presentationKey": "p:ccb9dce983a2a06796a6fe239525c260083d0bcc47c1c85cb4af671d74d8fc1a",
        "reference": 7,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Slot",
              "reference": 17
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "items",
            "members": [
              6
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": "inout",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "inBox",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:8230d75f47bdfaf7e0df502e00c52a2dd6978880f5cc262d441db1b3a93817ae",
        "reference": 15,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BoxPort",
              "reference": 8
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              7
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": "inout",
        "featureDirection": null,
        "metaclass": "ItemUsage",
        "multiplicity": null,
        "name": "box",
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:ae7f9d5cd355fb9a6c506178d902dd07ebf384a957d44121b27673fa73f9f558",
        "reference": 9,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Box",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "slot",
        "notationRole": "usage",
        "owner": 6,
        "presentationKey": "p:252107a530edc3a052d75862649ed0f49fc90491871cc93fd3fb2d1a7e5b6c96",
        "reference": 7,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Slot",
              "reference": 17
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              9
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "src",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:957b82c1046a05386337bb6a79d7523963e0ce2773549731762f42a9cea59dca",
        "reference": 5,
        "source": 19,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Source",
              "reference": 18
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "out1",
        "notationRole": "usage",
        "owner": 8,
        "presentationKey": "p:05f55bde4b353b10bf636e5df392aa7dc09ed9f90269359664add9b5ada636ff",
        "reference": 19,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              11,
              12
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "bundle",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:acc67a887c25a75f85ac8244021635c7c1468e072af89001e501386f2bb50cc9",
        "reference": 2,
        "source": 15,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Bundle",
              "reference": 10
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "a",
        "notationRole": "usage",
        "owner": 10,
        "presentationKey": "p:b1c75e86d2f66557cca8834de5b73e7ddd04c7a77a18974d42046649b7698404",
        "reference": 11,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "b",
        "notationRole": "usage",
        "owner": 10,
        "presentationKey": "p:5e0977b29bc9653306c761c6d271419edf0db2a93350ceabb3bbc2e178aeecfb",
        "reference": 12,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              14,
              15
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "second",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d12c193f13a374c7dadf379771adaf4ea63292995d8a70908757a6465479ddd4",
        "reference": 4,
        "source": 17,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Bundle",
              "reference": 10
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "a",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:ea3c1da9a1cab691a3a10df93aeeb97908b3ddaa874417ad57ba76a113430e97",
        "reference": 11,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "b",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:45ff233ad092f9455ea5e1cb50db9ecf2b53936dadd3fe0d85c2c457be73dbf8",
        "reference": 12,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 16
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "memberAccessOperand",
        "navigation": 24,
        "provenance": "authored",
        "reference": 27,
        "source": 1,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 25,
        "provenance": "authored",
        "reference": 28,
        "source": 1,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 20,
        "provenance": "authored",
        "reference": 39,
        "source": 8,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 53,
        "source": 9,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 16,
        "provenance": "authored",
        "reference": 31,
        "source": 10,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 45,
        "source": 11,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 47,
        "source": 12,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 18,
        "provenance": "authored",
        "reference": 37,
        "source": 13,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 46,
        "source": 14,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 48,
        "source": 15,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 22,
        "provenance": "authored",
        "reference": 34,
        "source": 2,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 52,
        "source": 5,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 43,
        "source": 6,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 40,
        "source": 7,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 13,
        "provenance": "authored",
        "reference": 50,
        "source": 3,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 41,
        "source": 4,
        "target": {
          "reference": 17,
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
