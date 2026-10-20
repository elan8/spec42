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
  "schemaVersion": 6,
  "modelDigest": "blake3:1a54ecda53ee22b69a435048bebac1462d4dadbb026481ecff5e1fdd4640dc8f",
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Assembly"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Assembly::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Assembly::bundle"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Assembly::destination"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Assembly::second"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Assembly::src"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Box"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Box::slot"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::BoxPort"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::BoxPort::box"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Bundle"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Bundle::a"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Bundle::b"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Destination"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Destination::carried"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Destination::inBox"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::P"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Slot"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Source"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::Source::out1"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "NestedOwners::selected"
    },
    {
      "kind": "source-anchor",
      "source": 23,
      "ownerQualifiedName": "NestedOwners::Assembly",
      "metaclass": "BindingConnectorAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "typing",
      "ordinal": 41
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "typing",
      "ordinal": 27
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "typing",
      "ordinal": 52
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "typing",
      "ordinal": 59
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "typing",
      "ordinal": 49
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "connection",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "typing",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "typing",
      "ordinal": 31
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "typing",
      "ordinal": 22
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "typing",
      "ordinal": 36
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "containment",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "typing",
      "ordinal": 56
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "typing",
      "ordinal": 45
    },
    {
      "kind": "relationship",
      "source": 19,
      "relationshipKind": "typing",
      "ordinal": 9
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
    "kind": "interconnection-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 0,
        "metaclass": "PartDefinition",
        "notationRole": "definition",
        "name": "Assembly",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 14,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "direct",
            "members": [
              2,
              8
            ]
          },
          {
            "kind": "ports",
            "provenance": "direct",
            "members": [
              10,
              13
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              1
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
        "reference": 21,
        "metaclass": "BindingConnectorAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 23,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 3,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "destination",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 13,
              "label": "Destination"
            }
          ]
        },
        "owner": 0,
        "source": 21,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              5
            ]
          },
          {
            "kind": "items",
            "provenance": "inherited",
            "members": [
              3
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
        "reference": 14,
        "metaclass": "ItemUsage",
        "notationRole": "usage",
        "name": "carried",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Box"
            }
          ]
        },
        "owner": 2,
        "source": 12,
        "compartments": [
          {
            "kind": "parts",
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
        "reference": 7,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "slot",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 17,
              "label": "Slot"
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
        "featureDirection": null
      },
      {
        "reference": 15,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "inBox",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 8,
              "label": "BoxPort"
            }
          ]
        },
        "owner": 2,
        "source": 10,
        "compartments": [
          {
            "kind": "items",
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
        "featureDirection": "inout"
      },
      {
        "reference": 9,
        "metaclass": "ItemUsage",
        "notationRole": "usage",
        "name": "box",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Box"
            }
          ]
        },
        "owner": 5,
        "source": 2,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "inherited",
            "members": [
              7
            ]
          }
        ],
        "direction": "inout",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 7,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "slot",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 17,
              "label": "Slot"
            }
          ]
        },
        "owner": 6,
        "source": 0,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "src",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 18,
              "label": "Source"
            }
          ]
        },
        "owner": 0,
        "source": 19,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
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
        "reference": 19,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "out1",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 16,
              "label": "P"
            }
          ]
        },
        "owner": 8,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "out"
      },
      {
        "reference": 2,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "bundle",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 10,
              "label": "Bundle"
            }
          ]
        },
        "owner": 0,
        "source": 15,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              11,
              12
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
        "reference": 11,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "a",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 16,
              "label": "P"
            }
          ]
        },
        "owner": 10,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "out"
      },
      {
        "reference": 12,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "b",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 16,
              "label": "P"
            }
          ]
        },
        "owner": 10,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "out"
      },
      {
        "reference": 4,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "second",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 10,
              "label": "Bundle"
            }
          ]
        },
        "owner": 0,
        "source": 17,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              14,
              15
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
        "reference": 11,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "a",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 16,
              "label": "P"
            }
          ]
        },
        "owner": 13,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "out"
      },
      {
        "reference": 12,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "b",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 16,
              "label": "P"
            }
          ]
        },
        "owner": 13,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": "out"
      }
    ],
    "relationships": [
      {
        "reference": 27,
        "source": 1,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 11
        },
        "provenance": "authored",
        "navigation": 24
      },
      {
        "reference": 28,
        "source": 1,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 9
        },
        "provenance": "authored",
        "navigation": 25
      },
      {
        "reference": 39,
        "source": 8,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 18
        },
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 53,
        "source": 9,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 16
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 31,
        "source": 10,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 10
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 45,
        "source": 11,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 16
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 47,
        "source": 12,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 16
        },
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 37,
        "source": 13,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 10
        },
        "provenance": "authored",
        "navigation": 18
      },
      {
        "reference": 46,
        "source": 14,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 16
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 48,
        "source": 15,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 16
        },
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 34,
        "source": 2,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 13
        },
        "provenance": "authored",
        "navigation": 22
      },
      {
        "reference": 52,
        "source": 5,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 8
        },
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 43,
        "source": 6,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 40,
        "source": 7,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 17
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 50,
        "source": 3,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 41,
        "source": 4,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 17
        },
        "provenance": "authored",
        "navigation": 1
      }
    ],
    "edges": [
      {
        "reference": 24,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 44,
        "source": 11,
        "target": 9,
        "origin": 1,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 24
      },
      {
        "reference": 22,
        "source": 0,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 38,
        "source": 8,
        "target": 9,
        "origin": 9,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 25,
        "source": 0,
        "target": 10,
        "origin": 10,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 29,
        "source": 10,
        "target": 11,
        "origin": 11,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 4
      },
      {
        "reference": 30,
        "source": 10,
        "target": 12,
        "origin": 12,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 6
      },
      {
        "reference": 26,
        "source": 0,
        "target": 13,
        "origin": 13,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 17
      },
      {
        "reference": 35,
        "source": 13,
        "target": 14,
        "origin": 14,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 4
      },
      {
        "reference": 36,
        "source": 13,
        "target": 15,
        "origin": 15,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 6
      },
      {
        "reference": 23,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 21
      },
      {
        "reference": 32,
        "source": 2,
        "target": 5,
        "origin": 5,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 10
      },
      {
        "reference": 51,
        "source": 5,
        "target": 6,
        "origin": 6,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 2
      },
      {
        "reference": 42,
        "source": 6,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 0
      },
      {
        "reference": 33,
        "source": 2,
        "target": 3,
        "origin": 3,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 12
      },
      {
        "reference": 49,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 0
      }
    ],
    "metadata": {
      "parts": [
        0,
        2,
        4,
        7,
        8
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
      ],
      "connectors": []
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
