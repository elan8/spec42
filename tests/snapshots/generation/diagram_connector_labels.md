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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d93545af0fd65e1704797247b19c990525274582f8275f0924278d413d66c440") (admitted (standard-library 94)))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::a"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Links")) (named (kind part) (name "system")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces"))) (provenance implied))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Links::system::mainPower"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
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
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink::b")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
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
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
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
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
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
      (type (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::DataLink")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any))
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
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
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
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:a92845afd051a11b40491a0dce29d0756b164e483fb8d4e8aafc6691701f82f5",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/connections.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/interfaces.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/occurrences.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/ports.md",
      "sourceDomain": "standard-library"
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
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Connections::BinaryConnection::source"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Connections::BinaryConnection::target"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Connections::connections"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Interfaces::interfaces"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::suboccurrences"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part::ownedPorts"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "Ports::ports"
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
      "ordinal": 25,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 48,
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
      "ordinal": 29,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 64,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "redefinition",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "redefinition",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 76,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 77,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 74,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "redefinition",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 79,
      "relationshipKind": "redefinition",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 81,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 78,
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
      "ordinal": 0,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "connectorEnd",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "connectorEnd",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 82,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 83,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 71,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 72,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 84,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 85,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 73,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 86,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "typeFeaturing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "subsetting",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "subsetting",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "subsetting",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "subsetting",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "typeFeaturing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 16
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
      "ordinal": 4,
      "relationshipKind": "subsetting",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "subsetting",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typeFeaturing",
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
      "ordinal": 15,
      "relationshipKind": "memberAccessOperand",
      "source": 17
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
      "relationshipKind": "subsetting",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "subsetting",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typeFeaturing",
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
        "reference": 80,
        "source": 0,
        "target": 5
      },
      {
        "kind": "connector",
        "navigation": 28,
        "origin": 5,
        "provenance": "authored",
        "reference": 44,
        "source": 14,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 6,
        "provenance": "implied",
        "reference": 128,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 7,
        "provenance": "implied",
        "reference": 129,
        "source": 5,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 30,
        "origin": 4,
        "provenance": "authored",
        "reference": 81,
        "source": 0,
        "target": 4
      },
      {
        "kind": "connector",
        "navigation": 31,
        "origin": 4,
        "provenance": "authored",
        "reference": 29,
        "source": 10,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 9,
        "provenance": "authored",
        "reference": 82,
        "source": 0,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 10,
        "provenance": "implied",
        "reference": 110,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 11,
        "provenance": "implied",
        "reference": 111,
        "source": 9,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 12,
        "provenance": "authored",
        "reference": 83,
        "source": 0,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 13,
        "provenance": "implied",
        "reference": 116,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 14,
        "provenance": "implied",
        "reference": 117,
        "source": 12,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 15,
        "provenance": "authored",
        "reference": 84,
        "source": 0,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 16,
        "provenance": "implied",
        "reference": 122,
        "source": 15,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 17,
        "provenance": "implied",
        "reference": 123,
        "source": 15,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 18,
        "provenance": "authored",
        "reference": 85,
        "source": 0,
        "target": 18
      },
      {
        "kind": "connector",
        "navigation": 35,
        "origin": 18,
        "provenance": "authored",
        "reference": 30,
        "source": 13,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 19,
        "provenance": "implied",
        "reference": 91,
        "source": 18,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 20,
        "provenance": "implied",
        "reference": 92,
        "source": 18,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 22,
        "origin": 1,
        "provenance": "authored",
        "reference": 86,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 24,
        "origin": 1,
        "provenance": "authored",
        "reference": 45,
        "source": 11,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 2,
        "provenance": "implied",
        "reference": 93,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 3,
        "provenance": "implied",
        "reference": 94,
        "source": 1,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 8,
        "provenance": "authored",
        "reference": 87,
        "source": 0,
        "target": 8
      },
      {
        "kind": "connector",
        "navigation": 20,
        "origin": 8,
        "provenance": "authored",
        "reference": 31,
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
        "metaclass": "PartUsage",
        "name": "system",
        "notationRole": "usage",
        "owner": null,
        "reference": 11,
        "source": 12,
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
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 27,
        "source": 22,
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
        "metaclass": "ConnectionUsage",
        "name": "source",
        "notationRole": "usage",
        "owner": 1,
        "reference": 7,
        "source": 0,
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
        "metaclass": "ConnectionUsage",
        "name": "target",
        "notationRole": "usage",
        "owner": 1,
        "reference": 8,
        "source": 2,
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
        "metaclass": "ConnectionUsage",
        "name": "namedUntyped",
        "notationRole": "usage",
        "owner": 0,
        "reference": 17,
        "source": 30,
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
        "metaclass": "ConnectionUsage",
        "name": "mainPower",
        "notationRole": "usage",
        "owner": 0,
        "reference": 16,
        "source": 26,
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
        "metaclass": "ConnectionUsage",
        "name": "source",
        "notationRole": "usage",
        "owner": 5,
        "reference": 7,
        "source": 0,
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
        "metaclass": "ConnectionUsage",
        "name": "target",
        "notationRole": "usage",
        "owner": 5,
        "reference": 8,
        "source": 2,
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
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 26,
        "source": 19,
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
        "metaclass": "PartUsage",
        "name": "a",
        "notationRole": "usage",
        "owner": 0,
        "reference": 13,
        "source": 13,
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
        "metaclass": "PortUsage",
        "name": "p",
        "notationRole": "usage",
        "owner": 9,
        "reference": 1,
        "source": 8,
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
        "metaclass": "PortUsage",
        "name": "q",
        "notationRole": "usage",
        "owner": 9,
        "reference": 2,
        "source": 10,
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
        "metaclass": "PartUsage",
        "name": "b",
        "notationRole": "usage",
        "owner": 0,
        "reference": 14,
        "source": 15,
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
        "metaclass": "PortUsage",
        "name": "p",
        "notationRole": "usage",
        "owner": 12,
        "reference": 1,
        "source": 8,
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
        "metaclass": "PortUsage",
        "name": "q",
        "notationRole": "usage",
        "owner": 12,
        "reference": 2,
        "source": 10,
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
        "metaclass": "PartUsage",
        "name": "c",
        "notationRole": "usage",
        "owner": 0,
        "reference": 15,
        "source": 17,
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
        "metaclass": "PortUsage",
        "name": "p",
        "notationRole": "usage",
        "owner": 15,
        "reference": 1,
        "source": 8,
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
        "metaclass": "PortUsage",
        "name": "q",
        "notationRole": "usage",
        "owner": 15,
        "reference": 2,
        "source": 10,
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
        "metaclass": "InterfaceUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 28,
        "source": 33,
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
        "metaclass": "ConnectionUsage",
        "name": "a",
        "notationRole": "usage",
        "owner": 18,
        "reference": 4,
        "source": 4,
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
        "metaclass": "ConnectionUsage",
        "name": "b",
        "notationRole": "usage",
        "owner": 18,
        "reference": 5,
        "source": 6,
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
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 88,
        "source": 0,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 27,
        "provenance": "authored",
        "reference": 135,
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
        "reference": 130,
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
        "reference": 131,
        "source": 5,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 132,
        "source": 5,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 133,
        "source": 5,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 134,
        "source": 5,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 70,
        "source": 6,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 64,
        "source": 6,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 66,
        "source": 6,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 68,
        "source": 6,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 78,
        "source": 7,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 72,
        "source": 7,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 74,
        "source": 7,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 76,
        "source": 7,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 31,
        "provenance": "authored",
        "reference": 136,
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
        "reference": 137,
        "source": 4,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 138,
        "source": 4,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 139,
        "source": 4,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 140,
        "source": 4,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 14,
        "provenance": "authored",
        "reference": 115,
        "source": 9,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 112,
        "source": 9,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 113,
        "source": 9,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 114,
        "source": 9,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 41,
        "source": 10,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 32,
        "source": 10,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
        "source": 10,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 38,
        "source": 10,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 55,
        "source": 11,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 46,
        "source": 11,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 47,
        "source": 11,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 52,
        "source": 11,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 16,
        "provenance": "authored",
        "reference": 121,
        "source": 12,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 118,
        "source": 12,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 119,
        "source": 12,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 120,
        "source": 12,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 42,
        "source": 13,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 34,
        "source": 13,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 35,
        "source": 13,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 39,
        "source": 13,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 56,
        "source": 14,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 48,
        "source": 14,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 49,
        "source": 14,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 53,
        "source": 14,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 18,
        "provenance": "authored",
        "reference": 127,
        "source": 15,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 124,
        "source": 15,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 125,
        "source": 15,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 126,
        "source": 15,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 43,
        "source": 16,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 36,
        "source": 16,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 37,
        "source": 16,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 40,
        "source": 16,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 57,
        "source": 17,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 50,
        "source": 17,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 51,
        "source": 17,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 54,
        "source": 17,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 35,
        "provenance": "authored",
        "reference": 89,
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
        "reference": 90,
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
        "reference": 107,
        "source": 18,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 108,
        "source": 18,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 99,
        "source": 18,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 104,
        "source": 18,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 60,
        "source": 19,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 58,
        "source": 19,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 59,
        "source": 19,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 63,
        "source": 20,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 61,
        "source": 20,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 20,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 23,
        "provenance": "authored",
        "reference": 109,
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
        "reference": 95,
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
        "reference": 96,
        "source": 1,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 100,
        "source": 1,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 101,
        "source": 1,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 105,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 71,
        "source": 2,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 65,
        "source": 2,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 67,
        "source": 2,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 69,
        "source": 2,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 79,
        "source": 3,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 73,
        "source": 3,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 75,
        "source": 3,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 77,
        "source": 3,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 20,
        "provenance": "authored",
        "reference": 97,
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
        "reference": 98,
        "source": 8,
        "target": {
          "node": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 102,
        "source": 8,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 103,
        "source": 8,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 106,
        "source": 8,
        "target": {
          "node": 0,
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
