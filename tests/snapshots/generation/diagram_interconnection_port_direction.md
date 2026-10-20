# META
~~~ini
description=Interconnection view publishes each port's authored direction and typing conjugation, resolved facts a renderer places port-l / port-r from rather than a guess
type=generate
libraries=standard
plugin=native:diagram
viewKind=interconnection-view
viewDocument=diagram_interconnection_port_direction.md
viewQualifiedName=PortDirectionExample::selected
~~~
# SOURCE
~~~sysml
package PortDirectionExample {
    private import StandardViewDefinitions::*;
    port def Signal;
    part def Node {
        in port inbound : Signal;
        out port outbound : Signal;
        inout port bidi : Signal;
        port plain : Signal;
        port mirrored : ~Signal;
    }
    view selected : InterconnectionView { expose Node; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_port_direction.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_port_direction.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:cc8975f107935792c036b6b216640ff7b07cbd68bee8708b4c71ac6a351e5283") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (kind port) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (kind port) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (kind port) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Signal") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Node")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))
      (type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))
      (type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))
      (type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))
      (type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))
      (type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 6 26) (end 6 32)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::bidi"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 4 26) (end 4 32)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::inbound"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 8 25) (end 8 31)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 5 28) (end 5 34)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 7 21) (end 7 27)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 10 20) (end 10 39)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_port_direction.md") (range (start 10 49) (end 10 53)) (probe (position 10 49))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (path (named (kind package) (name "PortDirectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:2cca878f99b34044cc5e4d6e7fa4adc007e6adb1aa957fbdbd36280425685243",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_interconnection_port_direction.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        16,
        4,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        26,
        4,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        17,
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
        34
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        19,
        6,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        26,
        6,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        13,
        7,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        21,
        7,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        25,
        8,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        9,
        10,
        17
      ]
    }
  ],
  "references": [
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::Node"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::Node::bidi"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::Node::inbound"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::Node::mirrored"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::Node::outbound"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::Node::plain"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::Signal"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "PortDirectionExample::selected"
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
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "typing",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "typing",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "typing",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 5
    }
  ],
  "selectedView": {
    "reference": 7,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 11
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
        "name": "Node",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 0,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "direct",
            "members": [
              1,
              2,
              3,
              4,
              5
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
        "name": "bidi",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Signal"
            }
          ]
        },
        "owner": 0,
        "source": 5,
        "compartments": [],
        "direction": "inout",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "plain",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Signal"
            }
          ]
        },
        "owner": 0,
        "source": 7,
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
        "name": "inbound",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Signal"
            }
          ]
        },
        "owner": 0,
        "source": 1,
        "compartments": [],
        "direction": "in",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 3,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "mirrored",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Signal"
            }
          ]
        },
        "owner": 0,
        "source": 9,
        "compartments": [],
        "direction": null,
        "conjugated": true,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 4,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "outbound",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Signal"
            }
          ]
        },
        "owner": 0,
        "source": 3,
        "compartments": [],
        "direction": "out",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 13,
        "source": 1,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 17,
        "source": 2,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 14,
        "source": 3,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 15,
        "source": 4,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 16,
        "source": 5,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 4
      }
    ],
    "edges": [
      {
        "reference": 8,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 9,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 10,
        "source": 0,
        "target": 3,
        "origin": 3,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 11,
        "source": 0,
        "target": 4,
        "origin": 4,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 12,
        "source": 0,
        "target": 5,
        "origin": 5,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 3
      }
    ],
    "metadata": {
      "parts": [
        0
      ],
      "ports": [
        1,
        2,
        3,
        4,
        5
      ],
      "connectors": []
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
