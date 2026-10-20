# META
~~~ini
description=Interconnection view projects nested parts and ports
type=generate
libraries=standard
plugin=native:diagram
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ce12c225d1a346c143964c73c962c77360e48d44e1f7035b4fda638062341a9d") (admitted (standard-library 94)))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
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
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature (resolved (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))))) (end bare (feature (resolved (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))))))
  (connector (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connection) (end bare (feature (resolved (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))))) (end bare (feature-chain (root (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (terminal (resolved (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap")))) (path (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested")) (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested::tap"))) "nested::tap")))
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
  "schemaVersion": 6,
  "modelDigest": "blake3:dface66026900a79323b21174b35e020a731624c8eccca5c8312aee2e4bcd680",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_interconnection_complete.md",
      "sourceDomain": "workspace"
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "InterconnectionExample::Assembly"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "InterconnectionExample::Assembly::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "InterconnectionExample::Assembly::input"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "InterconnectionExample::Assembly::nested"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "InterconnectionExample::Assembly::nested::tap"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "InterconnectionExample::Assembly::output"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "InterconnectionExample::selected"
    },
    {
      "kind": "source-anchor",
      "source": 5,
      "ownerQualifiedName": "InterconnectionExample::Assembly",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 8,
      "ownerQualifiedName": "InterconnectionExample::Assembly",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 6
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
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "connectorEnd",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "connectorEnd",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "connectorEnd",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 20
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "connection",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "connection",
      "ordinal": 7
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
        "source": 0,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "direct",
            "members": [
              3
            ]
          },
          {
            "kind": "ports",
            "provenance": "direct",
            "members": [
              5,
              6
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              1,
              2
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
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 5,
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
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 8,
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
        "name": "nested",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 3,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "direct",
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
        "reference": 4,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "tap",
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 4,
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
        "name": "input",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 1,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "output",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 2,
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
        "reference": 14,
        "source": 1,
        "kind": "connectorEnd",
        "target": {
          "status": "resolved",
          "node": 5
        },
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 15,
        "source": 1,
        "kind": "connectorEnd",
        "target": {
          "status": "resolved",
          "node": 6
        },
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 16,
        "source": 2,
        "kind": "connectorEnd",
        "target": {
          "status": "resolved",
          "node": 6
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 17,
        "source": 2,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 4
        },
        "provenance": "authored",
        "navigation": 10
      }
    ],
    "edges": [
      {
        "reference": 12,
        "source": 0,
        "target": 5,
        "origin": 5,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 9,
        "source": 0,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 19,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 13,
        "source": 0,
        "target": 6,
        "origin": 6,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 10,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 18,
        "source": 5,
        "target": 6,
        "origin": 1,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 11,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 20,
        "source": 6,
        "target": 4,
        "origin": 2,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 9
      }
    ],
    "metadata": {
      "parts": [
        0,
        3
      ],
      "ports": [
        4,
        5,
        6
      ],
      "connectors": [
        1,
        2
      ]
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
