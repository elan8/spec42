# META
~~~ini
description=A port without a direction of its own takes one from the directed features of its definition, reversed by a conjugated typing; an authored direction stays apart from it
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
    private import ScalarValues::*;
    port def Supply { out attribute volts : Real; }
    port def Bus { in attribute request : Real; out attribute reply : Real; }
    port def Marker;
    part def Source { port feed : Supply; port link : Bus; port tag : Marker; }
    part def Load { port feed : ~Supply; in port forced : Supply; }
    part system {
        part source : Source;
        part load : Load;
        connect source.feed to load.feed;
    }
    view links : InterconnectionView { expose system; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 16) (end 11 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3e3cebde663673d8b91dda13b76b7646117f9e3b805b9504261a4322eeb81919") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Bus")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Bus") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Supply") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (kind port) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Supply")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Marker")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Marker") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Supply")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bus")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Marker")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Supply")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Supply") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "source::feed")) (memberAccessOperand (reference "load::feed")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Load")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Source")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Supply")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (kind featureTyping) (ordinal 0))
      (authored-target "Supply")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Supply")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (kind featureTyping) (ordinal 0))
      (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "source::feed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "load::feed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (kind featureTyping) (ordinal 0))
      (authored-target "Load")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Bus")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Supply")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Marker")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Supply")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Supply")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Supply")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Supply")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind port-def) (name "Supply")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")) (scopes any))
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
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))) (terminal unresolved) (path (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) unresolved) "source::feed")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load")) (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) "load::feed")))
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
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 34)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 70) (end 4 74)) (probe (position 4 70))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::reply"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 42) (end 4 46)) (probe (position 4 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus::request"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 33) (end 7 39)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed"))) (kind featureTyping) (ordinal 0) (authored-target "Supply")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 58) (end 7 64)) (probe (position 7 58))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::forced"))) (kind featureTyping) (ordinal 0) (authored-target "Supply")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 34) (end 6 40)) (probe (position 6 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::feed"))) (kind featureTyping) (ordinal 0) (authored-target "Supply")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 54) (end 6 57)) (probe (position 6 54))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::link"))) (kind featureTyping) (ordinal 0) (authored-target "Bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Bus")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 70) (end 6 76)) (probe (position 6 70))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source::tag"))) (kind featureTyping) (ordinal 0) (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Marker")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 44) (end 3 48)) (probe (position 3 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Supply::volts"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 17) (end 13 36)) (probe (position 13 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::links"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 46) (end 13 52)) (probe (position 13 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 16) (end 11 27)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "source::feed")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 31) (end 11 40)) (probe (position 11 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Feed")) (named (kind part) (name "system")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "load::feed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load::feed")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 20) (end 10 24)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::load"))) (kind featureTyping) (ordinal 0) (authored-target "Load")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Load")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 22) (end 9 28)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::system::source"))) (kind featureTyping) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Feed::Source")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:84b7fd0cd1b910205d400f04bb781437a358a1af4e5d12d906a5c8f1028db9f8",
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
        6,
        27,
        6,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        34,
        6,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        47,
        6,
        51
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        54,
        6,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        64,
        6,
        67
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        70,
        6,
        76
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        25,
        7,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        33,
        7,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        49,
        7,
        55
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        58,
        7,
        64
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        9,
        8,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        22,
        9,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        13,
        10,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        20,
        10,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        16,
        11,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        31,
        11,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        9,
        13,
        14
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Bus"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Load"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Load::feed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Load::forced"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Marker"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Source"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Source::feed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Source::link"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Source::tag"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::Supply"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::links"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::system"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::system::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::system::load"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Feed::system::source"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Feed::system",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 18,
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
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "memberAccessOperand",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typing",
      "source": 14
    }
  ],
  "selectedView": {
    "reference": 10,
    "kind": "interconnection-view",
    "name": "links",
    "source": 18
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "relationship-unresolved",
        "relationshipKind": "connectorEnd"
      }
    ]
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 2,
        "provenance": "authored",
        "reference": 21,
        "source": 0,
        "target": 2
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 3,
        "provenance": "implied",
        "reference": 26,
        "source": 2,
        "target": 3
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 4,
        "provenance": "implied",
        "reference": 27,
        "source": 2,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 5,
        "provenance": "authored",
        "reference": 22,
        "source": 0,
        "target": 5
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 6,
        "provenance": "implied",
        "reference": 29,
        "source": 5,
        "target": 6
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 7,
        "provenance": "implied",
        "reference": 30,
        "source": 5,
        "target": 7
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 8,
        "provenance": "implied",
        "reference": 31,
        "source": 5,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 1,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 1
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
        2,
        5
      ],
      "portFamilies": [],
      "ports": [
        3,
        4,
        6,
        7,
        8
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              2,
              5
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
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "system",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:e8b9e5d923fa6a7ea1f7de806c95f399dd144c2b6524e581feeb28d4841723ec",
        "reference": 11,
        "source": 10,
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
        "presentationKey": "p:1c213894fe99fb36407c8429a8a0a36769f89069e41c9aa43a12712b86192c5a",
        "reference": 15,
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
              3,
              4
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "load",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:92b4edfe3e6b7b3f4c69dd9747af1ca8b209696cb97572d91503ae2f5de5e2c0",
        "reference": 13,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Load",
              "reference": 1
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": "in",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "feed",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:a8b1022ef49b4736240e25c041a2a6fe991478e0f3935fdf6bc2fc0f8f218a45",
        "reference": 2,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Supply",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "in",
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "forced",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:d5df3127305be1d65996edde2e1efbeb51ee508efea3f1a322d0dd0e578daeda",
        "reference": 3,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Supply",
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
              6,
              7,
              8
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "source",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:fbdc7f5321d57218ae9f728c44c0c1f55abd11de8995875dbbac5f1cca666e3f",
        "reference": 14,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Source",
              "reference": 5
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
        "name": "tag",
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:2d8bee4477febae6f801eaf68780525e645fd5f700f314b6aab26ef5dc727cdc",
        "reference": 8,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Marker",
              "reference": 4
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
        "name": "feed",
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:b2147c18f63285162d4fd56617ef8eba7f02d953c589e699a1635223002f88d7",
        "reference": 6,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Supply",
              "reference": 9
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
        "name": "link",
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:4f0f4c09de0f36cd96cff84d783f18838e5c34adc1ee2d7293b51599eb77766f",
        "reference": 7,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Bus",
              "reference": 0
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 14,
        "provenance": "authored",
        "reference": 28,
        "source": 2,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 16,
        "source": 3,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 17,
        "source": 4,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 32,
        "source": 5,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 20,
        "source": 6,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 18,
        "source": 7,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 19,
        "source": 8,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 16,
        "provenance": "authored",
        "reference": 24,
        "source": 1,
        "target": {
          "status": "unresolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 17,
        "provenance": "authored",
        "reference": 25,
        "source": 1,
        "target": {
          "node": 3,
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
