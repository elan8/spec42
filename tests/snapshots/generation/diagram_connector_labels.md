# META
~~~ini
description=Connectors are labelled with their authored name, else their type; anonymous untyped ones are unlabelled
type=generate
libraries=standard
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=Links::links
~~~
# SOURCE
## model.sysml
~~~sysml
package Links {
    private import StandardViewDefinitions::*;
    port def PowerPort;
    connection def PowerLink { end source : PowerPort; end target : PowerPort; }
    interface def DataLink { end a : PowerPort; end b : PowerPort; }
    part def Box { port p : PowerPort; port q : PowerPort; }
    part system {
        part a : Box;
        part b : Box;
        part c : Box;
        connect a.p to b.p;
        connection : PowerLink connect a.q to b.q;
        connection mainPower : PowerLink connect b.q to c.q;
        connection namedUntyped connect a.p to c.p;
        interface : DataLink connect b.p to c.p;
    }
    view links : InterconnectionView { expose system; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:0525682b4202320ed1e6cf33f61e7d9954301fc8f72b8f15d9d1d4cc153b0a51") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~PowerPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "a::p")) (memberAccessOperand (reference "b::p")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerLink")) (memberAccessOperand (reference "a::q")) (memberAccessOperand (reference "b::q")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DataLink")) (connectorEnd (reference "b::p")) (connectorEnd (reference "c::p")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerLink")) (memberAccessOperand (reference "b::q")) (memberAccessOperand (reference "c::q")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "a::p")) (memberAccessOperand (reference "c::p")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "DataLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "b::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "c::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "a::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "b::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "c::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "c::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")))
      (positional-ends (authored 2) (effective 2))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))
      (positional-ends (authored 2) (effective 2))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")))
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
      (subtype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))))
      (effective-type (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::binaryInterfaces")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (kind interface) (end (name (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) unconnected) (end (name (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) unconnected))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink"))) (kind connection) (end (name (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) unconnected) (end (name (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) unconnected))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) "a::p")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) "b::p")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (type (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) "a::q")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) "b::q")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind interface) (type (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")))) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) "b::p")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) "c::p")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind connection) (type (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) "b::q")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) "c::q")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) "a::p")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c")) (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) "c::p")))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 28) (end 5 37)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 48) (end 5 57)) (probe (position 5 48))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 37) (end 4 46)) (probe (position 4 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 56) (end 4 65)) (probe (position 4 56))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 44) (end 3 53)) (probe (position 3 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::source"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 68) (end 3 77)) (probe (position 3 68))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink::target"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 17) (end 16 36)) (probe (position 16 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 46) (end 16 52)) (probe (position 16 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 21) (end 11 30)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "PowerLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 20) (end 14 28)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "DataLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 37) (end 14 40)) (probe (position 14 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "b::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 44) (end 14 47)) (probe (position 14 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "c::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 16) (end 10 19)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 39) (end 11 42)) (probe (position 11 39))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "a::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 23) (end 10 26)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "b::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 46) (end 11 49)) (probe (position 11 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 17) (end 7 20)) (probe (position 7 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::a"))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 17) (end 8 20)) (probe (position 8 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::b"))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 17) (end 9 20)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::c"))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 31) (end 12 40)) (probe (position 12 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind featureTyping) (ordinal 0) (authored-target "PowerLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerLink")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 49) (end 12 52)) (probe (position 12 49))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind memberAccessOperand) (ordinal 0) (authored-target "b::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 56) (end 12 59)) (probe (position 12 56))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (kind memberAccessOperand) (ordinal 1) (authored-target "c::q")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::q")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 40) (end 13 43)) (probe (position 13 40))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind memberAccessOperand) (ordinal 0) (authored-target "a::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 47) (end 13 50)) (probe (position 13 47))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::namedUntyped"))) (kind memberAccessOperand) (ordinal 1) (authored-target "c::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::Box::p")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:f60865da532fb4806f880a77d3203f7ca7f058fb71ace600b449bb2f812d2111",
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
        3,
        35,
        3,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        44,
        3,
        53
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        59,
        3,
        65
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        68,
        3,
        77
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        33,
        4,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        37,
        4,
        46
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        52,
        4,
        53
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        56,
        4,
        65
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        24,
        5,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        28,
        5,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        44,
        5,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        48,
        5,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        9,
        6,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        13,
        7,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        17,
        7,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        17,
        8,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        17,
        9,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        8,
        10,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        16,
        10,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        23,
        10,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        50
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        21,
        11,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        39,
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
        49
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        19,
        12,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        31,
        12,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        49,
        12,
        52
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        56,
        12,
        59
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        19,
        13,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        40,
        13,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        47,
        13,
        50
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        8,
        14,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        20,
        14,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        37,
        14,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        44,
        14,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        9,
        16,
        14
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::Box"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::Box::p"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::Box::q"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::DataLink"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::DataLink::a"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::DataLink::b"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::PowerLink"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::PowerLink::source"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::PowerLink::target"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::PowerPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::links"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::system"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::system::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::system::a"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::system::b"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::system::c"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::system::mainPower"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Links::system::namedUntyped"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Links::system",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Links::system",
      "source": 22,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "InterfaceUsage",
      "ownerQualifiedName": "Links::system",
      "source": 33,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "connection",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "connection",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "connection",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "connection",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "connection",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 84,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "connectorEnd",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "connectorEnd",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 74,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 88,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 89,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 73,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "noncomposite-membership",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "noncomposite-membership",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "typing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "noncomposite-membership",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "memberAccessOperand",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "memberAccessOperand",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "noncomposite-membership",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "memberAccessOperand",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "memberAccessOperand",
      "source": 17
    }
  ],
  "selectedView": {
    "reference": 10,
    "kind": "interconnection-view",
    "name": "links",
    "source": 37
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 26,
        "origin": 5,
        "provenance": "authored",
        "reference": 38,
        "source": 0,
        "target": 5
      },
      {
        "kind": "connector",
        "navigation": 28,
        "origin": 5,
        "provenance": "authored",
        "reference": 27,
        "source": 14,
        "target": 17
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 6,
        "provenance": "implied",
        "reference": 69,
        "source": 5,
        "target": 6
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 7,
        "provenance": "implied",
        "reference": 70,
        "source": 5,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 30,
        "origin": 4,
        "provenance": "authored",
        "reference": 39,
        "source": 0,
        "target": 4
      },
      {
        "kind": "connector",
        "navigation": 31,
        "origin": 4,
        "provenance": "authored",
        "reference": 21,
        "source": 10,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 9,
        "provenance": "authored",
        "reference": 40,
        "source": 0,
        "target": 9
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 10,
        "provenance": "implied",
        "reference": 58,
        "source": 9,
        "target": 10
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 11,
        "provenance": "implied",
        "reference": 59,
        "source": 9,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 12,
        "provenance": "authored",
        "reference": 41,
        "source": 0,
        "target": 12
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 13,
        "provenance": "implied",
        "reference": 61,
        "source": 12,
        "target": 13
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 14,
        "provenance": "implied",
        "reference": 62,
        "source": 12,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 15,
        "provenance": "authored",
        "reference": 42,
        "source": 0,
        "target": 15
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 16,
        "provenance": "implied",
        "reference": 64,
        "source": 15,
        "target": 16
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 17,
        "provenance": "implied",
        "reference": 65,
        "source": 15,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 18,
        "provenance": "authored",
        "reference": 43,
        "source": 0,
        "target": 18
      },
      {
        "kind": "connector",
        "navigation": 35,
        "origin": 18,
        "provenance": "authored",
        "reference": 22,
        "source": 13,
        "target": 16
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 19,
        "provenance": "implied",
        "reference": 52,
        "source": 18,
        "target": 19
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 20,
        "provenance": "implied",
        "reference": 53,
        "source": 18,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 22,
        "origin": 1,
        "provenance": "authored",
        "reference": 44,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 24,
        "origin": 1,
        "provenance": "authored",
        "reference": 28,
        "source": 11,
        "target": 14
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 2,
        "provenance": "implied",
        "reference": 54,
        "source": 1,
        "target": 2
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 3,
        "provenance": "implied",
        "reference": 55,
        "source": 1,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 8,
        "provenance": "authored",
        "reference": 45,
        "source": 0,
        "target": 8
      },
      {
        "kind": "connector",
        "navigation": 20,
        "origin": 8,
        "provenance": "authored",
        "reference": 23,
        "source": 10,
        "target": 13
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        19,
        20
      ],
      "parts": [
        0,
        9,
        12,
        15
      ],
      "portFamilies": [
        {
          "connectors": [
            1,
            4,
            5,
            8,
            18
          ],
          "general": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ],
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      ],
      "ports": [
        10,
        11,
        13,
        14,
        16,
        17
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              9,
              12,
              15
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1,
              4,
              5,
              8
            ],
            "provenance": "direct"
          },
          {
            "kind": "interfaces",
            "members": [
              18
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "system",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:55ac1fa56dc1edfb6da84a2950206d59af59ea1ba98f6cb083c8e7790b7ebbfc",
        "reference": 11,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "connections",
            "members": [
              2,
              3
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:6699a9248492b9a19315f72fa169ee53e892fa8b59f1fb4b56bf38bbdd6ca4a7",
        "reference": 19,
        "source": 22,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerLink",
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
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "source",
        "notationRole": "usage",
        "owner": 1,
        "presentationKey": "p:a94ccf625d0f7def92c097fa052a55ab1564833dd46b0b36c48202dcd4b94063",
        "reference": 7,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "target",
        "notationRole": "usage",
        "owner": 1,
        "presentationKey": "p:2980da6be66d7550a0144d218e4f3570237a4331abdf54cc5a919df564e59610",
        "reference": 8,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "namedUntyped",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:08cc2e1e5eab046549c53ac2f659568073484ae900057a7974cd5a49a5a3edd0",
        "reference": 17,
        "source": 30,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "connections",
            "members": [
              6,
              7
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "mainPower",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:16c4e5e7239d448a9532c46a3043b5064f6d9091fe008985ac7544ce43d56d18",
        "reference": 16,
        "source": 26,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerLink",
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
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "source",
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:29f47e38d7951a955e234977c40db287199063776bb69870a2bfbf22c7b6d370",
        "reference": 7,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "target",
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:5307a6f66d4015dd90872c9448936bff4f43847daff498cf910dd3a6326ec2a8",
        "reference": 8,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
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
        "presentationKey": "p:4d7de3551537108331f3b3522cc15c9ebfba1ea3a2025b91eb18a547b6ce913b",
        "reference": 18,
        "source": 19,
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
              10,
              11
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "a",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:8dc33706d2ac6c9071185991fc48355ad5484cf242c2ff97bbca8c2d0a2d0468",
        "reference": 13,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Box",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "p",
        "notationRole": "usage",
        "owner": 9,
        "presentationKey": "p:d81ca53bc1d9afa32f9b7acaf00c844d25c9d8cc605bdd7b123264f007d2bebc",
        "reference": 1,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "q",
        "notationRole": "usage",
        "owner": 9,
        "presentationKey": "p:73f56620a10ff5516ffaf3c0b5a6c18810b989a084e1f80929c939d273d6a5e5",
        "reference": 2,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              13,
              14
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "b",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:e1994b0408d3d2a5fd389fbd64f0d6fc6ddc229762554966683a5c198c874369",
        "reference": 14,
        "source": 15,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Box",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "p",
        "notationRole": "usage",
        "owner": 12,
        "presentationKey": "p:8f9d00f7ae26c1e398552777691d5b49990954842b6ec79f653ac008008cd3a6",
        "reference": 1,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "q",
        "notationRole": "usage",
        "owner": 12,
        "presentationKey": "p:9ca4ba6980ef52a497d4a9904108f606bfefd13bb8b4ae5503579f46750aeb28",
        "reference": 2,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              16,
              17
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "c",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:907d9fe4a66d51668bc0e305966bd1d4190d464a3ac00866ffe0bfed47c8749f",
        "reference": 15,
        "source": 17,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Box",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "p",
        "notationRole": "usage",
        "owner": 15,
        "presentationKey": "p:a909135d5fa46d2175a257aef604f83ae9043da3d44677dbab87a5418a5d6e35",
        "reference": 1,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "q",
        "notationRole": "usage",
        "owner": 15,
        "presentationKey": "p:f6237ee07a9be7d0978c4e312d1c52f9f2c10aefa865c4521e205fd80da75d1a",
        "reference": 2,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "connections",
            "members": [
              19,
              20
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "InterfaceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:5488e4142d93d80fd9e942cce44e16cf4185293cf23bcd179bdc07934ced385a",
        "reference": 20,
        "source": 33,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DataLink",
              "reference": 3
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "a",
        "notationRole": "usage",
        "owner": 18,
        "presentationKey": "p:d02fb3e9429410305c4e84ed566429974523caad1b4c7a09fa09904dfcabca5a",
        "reference": 4,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": "b",
        "notationRole": "usage",
        "owner": 18,
        "presentationKey": "p:f22a334b1236fad3d2a06369f8f992b9efa108bf45f08fcd4fec6ebbea5a296b",
        "reference": 5,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 9
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 27,
        "provenance": "authored",
        "reference": 71,
        "source": 5,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 28,
        "provenance": "authored",
        "reference": 67,
        "source": 5,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 29,
        "provenance": "authored",
        "reference": 68,
        "source": 5,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 34,
        "source": 6,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 36,
        "source": 7,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 31,
        "provenance": "authored",
        "reference": 72,
        "source": 4,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 32,
        "provenance": "authored",
        "reference": 73,
        "source": 4,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 14,
        "provenance": "authored",
        "reference": 60,
        "source": 9,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 24,
        "source": 10,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 29,
        "source": 11,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 16,
        "provenance": "authored",
        "reference": 63,
        "source": 12,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 25,
        "source": 13,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 30,
        "source": 14,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 18,
        "provenance": "authored",
        "reference": 66,
        "source": 15,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 26,
        "source": 16,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 31,
        "source": 17,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 35,
        "provenance": "authored",
        "reference": 46,
        "source": 18,
        "target": {
          "node": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 36,
        "provenance": "authored",
        "reference": 47,
        "source": 18,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 34,
        "provenance": "authored",
        "reference": 56,
        "source": 18,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 32,
        "source": 19,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 33,
        "source": 20,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 23,
        "provenance": "authored",
        "reference": 57,
        "source": 1,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 24,
        "provenance": "authored",
        "reference": 48,
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
        "reference": 49,
        "source": 1,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 35,
        "source": 2,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 37,
        "source": 3,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 20,
        "provenance": "authored",
        "reference": 50,
        "source": 8,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 21,
        "provenance": "authored",
        "reference": 51,
        "source": 8,
        "target": {
          "node": 13,
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
