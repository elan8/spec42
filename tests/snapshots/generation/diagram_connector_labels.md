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
  "schemaVersion": 6,
  "modelDigest": "blake3:c283bd408e63a62c86726a772afd355ce139817d31150672b94ef780f9802790",
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::Box"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::Box::p"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::Box::q"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::DataLink"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::DataLink::a"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::DataLink::b"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::PowerLink"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::PowerLink::source"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::PowerLink::target"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::PowerPort"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::links"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::system"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::system::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::system::a"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::system::b"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::system::c"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::system::mainPower"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Links::system::namedUntyped"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Links::system",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 22,
      "ownerQualifiedName": "Links::system",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 33,
      "ownerQualifiedName": "Links::system",
      "metaclass": "InterfaceUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "connection",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "connection",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "connection",
      "ordinal": 24
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "typing",
      "ordinal": 26
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "typing",
      "ordinal": 38
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "typing",
      "ordinal": 50
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "connection",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "connection",
      "ordinal": 20
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 30
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 42
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 54
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "typing",
      "ordinal": 65
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 69
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "typing",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "typing",
      "ordinal": 80
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "typing",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "typing",
      "ordinal": 84
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
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 23
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "connectorEnd",
      "ordinal": 58
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "connectorEnd",
      "ordinal": 59
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 74
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 75
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 88
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 89
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 21
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 22
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "typing",
      "ordinal": 60
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "typing",
      "ordinal": 73
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "typing",
      "ordinal": 22
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "typing",
      "ordinal": 34
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "typing",
      "ordinal": 46
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "typing",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 17,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 17,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 17
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
    "kind": "interconnection-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 11,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "system",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 12,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "direct",
            "members": [
              9,
              12,
              15
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              1,
              4,
              5,
              8
            ]
          },
          {
            "kind": "interfaces",
            "provenance": "direct",
            "members": [
              18
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
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "PowerLink"
            }
          ]
        },
        "owner": 0,
        "source": 22,
        "compartments": [
          {
            "kind": "connections",
            "provenance": "inherited",
            "members": [
              2,
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
        "reference": 7,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "source",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 1,
        "source": 0,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "target",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 1,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 17,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "namedUntyped",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 30,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 16,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "mainPower",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "PowerLink"
            }
          ]
        },
        "owner": 0,
        "source": 26,
        "compartments": [
          {
            "kind": "connections",
            "provenance": "inherited",
            "members": [
              6,
              7
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
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "source",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
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
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "target",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 5,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 18,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 19,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 13,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "a",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Box"
            }
          ]
        },
        "owner": 0,
        "source": 13,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              10,
              11
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
        "name": "p",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 9,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "q",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 9,
        "source": 10,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 14,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "b",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Box"
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
              13,
              14
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
        "name": "p",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 12,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "q",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 12,
        "source": 10,
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
        "name": "c",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Box"
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
              16,
              17
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
        "name": "p",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 15,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "q",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 15,
        "source": 10,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 20,
        "metaclass": "InterfaceUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 3,
              "label": "DataLink"
            }
          ]
        },
        "owner": 0,
        "source": 33,
        "compartments": [
          {
            "kind": "connections",
            "provenance": "inherited",
            "members": [
              19,
              20
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
        "reference": 4,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "a",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 18,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": "b",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "PowerPort"
            }
          ]
        },
        "owner": 18,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 71,
        "source": 5,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 27
      },
      {
        "reference": 67,
        "source": 5,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 14
        },
        "provenance": "authored",
        "navigation": 28
      },
      {
        "reference": 68,
        "source": 5,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 29
      },
      {
        "reference": 34,
        "source": 6,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 36,
        "source": 7,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 72,
        "source": 4,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 10
        },
        "provenance": "authored",
        "navigation": 31
      },
      {
        "reference": 73,
        "source": 4,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 16
        },
        "provenance": "authored",
        "navigation": 32
      },
      {
        "reference": 60,
        "source": 9,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 24,
        "source": 10,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 29,
        "source": 11,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 63,
        "source": 12,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 25,
        "source": 13,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 30,
        "source": 14,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 66,
        "source": 15,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 18
      },
      {
        "reference": 26,
        "source": 16,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 31,
        "source": 17,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 46,
        "source": 18,
        "kind": "connectorEnd",
        "target": {
          "status": "resolved",
          "node": 13
        },
        "provenance": "authored",
        "navigation": 35
      },
      {
        "reference": 47,
        "source": 18,
        "kind": "connectorEnd",
        "target": {
          "status": "resolved",
          "node": 16
        },
        "provenance": "authored",
        "navigation": 36
      },
      {
        "reference": 56,
        "source": 18,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 3
        },
        "provenance": "authored",
        "navigation": 34
      },
      {
        "reference": 32,
        "source": 19,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 33,
        "source": 20,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 57,
        "source": 1,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 48,
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
        "reference": 49,
        "source": 1,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 14
        },
        "provenance": "authored",
        "navigation": 25
      },
      {
        "reference": 35,
        "source": 2,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 37,
        "source": 3,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 50,
        "source": 8,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 10
        },
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 51,
        "source": 8,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 13
        },
        "provenance": "authored",
        "navigation": 21
      }
    ],
    "edges": [
      {
        "reference": 38,
        "source": 0,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 26
      },
      {
        "reference": 27,
        "source": 14,
        "target": 17,
        "origin": 5,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 28
      },
      {
        "reference": 69,
        "source": 5,
        "target": 6,
        "origin": 6,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 0
      },
      {
        "reference": 70,
        "source": 5,
        "target": 7,
        "origin": 7,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 2
      },
      {
        "reference": 39,
        "source": 0,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 30
      },
      {
        "reference": 21,
        "source": 10,
        "target": 16,
        "origin": 4,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 31
      },
      {
        "reference": 40,
        "source": 0,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 58,
        "source": 9,
        "target": 10,
        "origin": 10,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 59,
        "source": 9,
        "target": 11,
        "origin": 11,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 10
      },
      {
        "reference": 41,
        "source": 0,
        "target": 12,
        "origin": 12,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 61,
        "source": 12,
        "target": 13,
        "origin": 13,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 62,
        "source": 12,
        "target": 14,
        "origin": 14,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 10
      },
      {
        "reference": 42,
        "source": 0,
        "target": 15,
        "origin": 15,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 17
      },
      {
        "reference": 64,
        "source": 15,
        "target": 16,
        "origin": 16,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 65,
        "source": 15,
        "target": 17,
        "origin": 17,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 10
      },
      {
        "reference": 43,
        "source": 0,
        "target": 18,
        "origin": 18,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 33
      },
      {
        "reference": 22,
        "source": 13,
        "target": 16,
        "origin": 18,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 35
      },
      {
        "reference": 52,
        "source": 18,
        "target": 19,
        "origin": 19,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 4
      },
      {
        "reference": 53,
        "source": 18,
        "target": 20,
        "origin": 20,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 6
      },
      {
        "reference": 44,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 22
      },
      {
        "reference": 28,
        "source": 11,
        "target": 14,
        "origin": 1,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 24
      },
      {
        "reference": 54,
        "source": 1,
        "target": 2,
        "origin": 2,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 0
      },
      {
        "reference": 55,
        "source": 1,
        "target": 3,
        "origin": 3,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 2
      },
      {
        "reference": 45,
        "source": 0,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 23,
        "source": 10,
        "target": 13,
        "origin": 8,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 20
      }
    ],
    "metadata": {
      "parts": [
        0,
        9,
        12,
        15
      ],
      "ports": [
        10,
        11,
        13,
        14,
        16,
        17
      ],
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
      ]
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
