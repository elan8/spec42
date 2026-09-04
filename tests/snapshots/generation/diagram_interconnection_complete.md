# META
~~~ini
description=Interconnection view projects nested parts and ports
type=generate
libraries=standard
plugin=repository:diagram
viewKind=interconnection-view
viewDocument=diagram_interconnection_complete.md
viewQualifiedName=InterconnectionExample::selected
~~~
# SOURCE
~~~sysml
package InterconnectionExample {
    private import StandardViewDefinitions::*;
    part def Assembly {
        port input;
        port output;
        part nested { port tap; attribute note; }
        // Plain and dotted connector ends both compose a `connection` edge between ports.
        connect input to output;
        connect output to nested.tap;
    }
    view selected : InterconnectionView { expose Assembly; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_complete.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_complete.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f107a429784874c5cb2122086463a763c006eaa1eb5f45445542daa8280046af") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "input")) (connectorEnd (reference "output")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "output")) (memberAccessOperand (reference "nested::tap")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::note"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Assembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "output")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "output")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "nested::tap")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::note"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::note"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (provenance implied))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (provenance implied))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::note")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 8 16) (end 8 22)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "output")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 7 25) (end 7 31)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "output")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 8 26) (end 8 36)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "nested::tap")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 10 20) (end 10 39)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 10 49) (end 10 57)) (probe (position 10 49))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:2db96f3e663d608546765689bd96fecef67559f43ebda8cff169e05b25d896af",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_interconnection_complete.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/connections.md",
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
        2,
        13,
        2,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        27,
        5,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        16,
        7,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        25,
        7,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        8,
        8,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        16,
        8,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        26,
        8,
        36
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
      "qualifiedName": "InterconnectionExample::Assembly"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InterconnectionExample::Assembly::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InterconnectionExample::Assembly::input"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InterconnectionExample::Assembly::nested"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InterconnectionExample::Assembly::nested::tap"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InterconnectionExample::Assembly::output"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InterconnectionExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Connections::connections"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part::ownedPorts"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Ports::ports"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "InterconnectionExample::Assembly",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "InterconnectionExample::Assembly",
      "source": 8,
      "sourceDomain": "workspace"
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
      "ordinal": 6,
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
      "ordinal": 12,
      "relationshipKind": "connectorEnd",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "connectorEnd",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "connectorEnd",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "memberAccessOperand",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "connection",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "connection",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "typeFeaturing",
      "source": 5
    }
  ],
  "selectedView": {
    "reference": 6,
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
        "navigation": 1,
        "origin": 5,
        "provenance": "authored",
        "reference": 14,
        "source": 0,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 3,
        "provenance": "authored",
        "reference": 15,
        "source": 0,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 4,
        "provenance": "authored",
        "reference": 32,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 6,
        "provenance": "authored",
        "reference": 16,
        "source": 0,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 1,
        "provenance": "authored",
        "reference": 17,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 6,
        "origin": 1,
        "provenance": "authored",
        "reference": 28,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 2,
        "provenance": "authored",
        "reference": 18,
        "source": 0,
        "target": 2
      },
      {
        "kind": "connector",
        "navigation": 9,
        "origin": 2,
        "provenance": "authored",
        "reference": 38,
        "source": 6,
        "target": 4
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [
        1,
        2
      ],
      "parts": [
        0,
        3
      ],
      "ports": [
        4,
        5,
        6
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              3
            ],
            "provenance": "direct"
          },
          {
            "kind": "ports",
            "members": [
              5,
              6
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1,
              2
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "metaclass": "PartDefinition",
        "name": "Assembly",
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
        "direction": null,
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 12,
        "source": 5,
        "typing": {
          "status": "absent"
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
        "reference": 13,
        "source": 8,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              4
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "metaclass": "PartUsage",
        "name": "nested",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 3,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "PortUsage",
        "name": "tap",
        "notationRole": "usage",
        "owner": 3,
        "reference": 4,
        "source": 4,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "PortUsage",
        "name": "input",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "PortUsage",
        "name": "output",
        "notationRole": "usage",
        "owner": 0,
        "reference": 5,
        "source": 2,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 19,
        "source": 0,
        "target": {
          "reference": 8,
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
          "reference": 11,
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
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
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
        "reference": 34,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 35,
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
        "reference": 36,
        "source": 4,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 37,
        "source": 4,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 39,
        "source": 6,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 40,
        "source": 6,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 41,
        "source": 6,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 6,
        "provenance": "authored",
        "reference": 20,
        "source": 1,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 7,
        "provenance": "authored",
        "reference": 21,
        "source": 1,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 26,
        "source": 1,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 24,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 9,
        "provenance": "authored",
        "reference": 22,
        "source": 2,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 27,
        "source": 2,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 10,
        "provenance": "authored",
        "reference": 23,
        "source": 2,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 25,
        "source": 2,
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
