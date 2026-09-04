# META
~~~ini
description=Interconnection view publishes each port's authored direction and typing conjugation, resolved facts a renderer places port-l / port-r from rather than a guess
type=generate
libraries=standard
plugin=repository:diagram
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
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 4 8) (end 4 33))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 5 8) (end 5 35))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 6 8) (end 6 33))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 7 8) (end 7 28))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 8 8) (end 8 32))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_port_direction.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 4 8) (end 4 33))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 5 8) (end 5 35))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 6 8) (end 6 33))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 7 8) (end 7 28))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 8 8) (end 8 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:af39ac3b3beca23419486ae811159e0b31090f0c395503b5769b2e8b8fa6e1b8") (admitted (standard-library 94)))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::mirrored"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::outbound"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (target (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Node::plain"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_port_direction.md") (qualified-name "PortDirectionExample::Signal"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:a3a2e5c6a1440c85c2a8e492307eee40e6074f97e6e3699cd3f3c6d33f8e9f7f",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_interconnection_port_direction.md",
      "sourceDomain": "workspace"
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
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::Node"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::Node::bidi"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::Node::inbound"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::Node::mirrored"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::Node::outbound"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::Node::plain"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::Signal"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "PortDirectionExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part::ownedPorts"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Ports::ports"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 5
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
    "edges": [
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 1,
        "provenance": "authored",
        "reference": 11,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 2,
        "provenance": "authored",
        "reference": 12,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 3,
        "provenance": "authored",
        "reference": 13,
        "source": 0,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 4,
        "provenance": "authored",
        "reference": 14,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 5,
        "provenance": "authored",
        "reference": 15,
        "source": 0,
        "target": 5
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [],
      "parts": [
        0
      ],
      "ports": [
        1,
        2,
        3,
        4,
        5
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              1,
              2,
              3,
              4,
              5
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "metaclass": "PartDefinition",
        "name": "Node",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 0,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "inout",
        "metaclass": "PortUsage",
        "name": "bidi",
        "notationRole": "usage",
        "owner": 0,
        "reference": 1,
        "source": 5,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "PortUsage",
        "name": "plain",
        "notationRole": "usage",
        "owner": 0,
        "reference": 5,
        "source": 7,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "in",
        "metaclass": "PortUsage",
        "name": "inbound",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 1,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "metaclass": "PortUsage",
        "name": "mirrored",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 9,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "out",
        "metaclass": "PortUsage",
        "name": "outbound",
        "notationRole": "usage",
        "owner": 0,
        "reference": 4,
        "source": 3,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 6
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 16,
        "source": 0,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 20,
        "source": 1,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 17,
        "source": 1,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 18,
        "source": 1,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 19,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 36,
        "source": 2,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
        "source": 2,
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
        "source": 2,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 35,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 24,
        "source": 3,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 21,
        "source": 3,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 22,
        "source": 3,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 23,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 10,
        "provenance": "authored",
        "reference": 28,
        "source": 4,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 25,
        "source": 4,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 26,
        "source": 4,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 27,
        "source": 4,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 32,
        "source": 5,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 29,
        "source": 5,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 30,
        "source": 5,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 31,
        "source": 5,
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
