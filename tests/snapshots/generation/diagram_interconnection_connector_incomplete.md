# META
~~~ini
description=Interconnection View records a typed incomplete reason for a connector whose end is unresolved or outside the projection, and never draws a guessed line
type=generate
libraries=standard
plugin=repository:diagram
viewKind=interconnection-view
viewDocument=diagram_interconnection_connector_incomplete.md
viewQualifiedName=ConnectorIncomplete::selected
~~~
# SOURCE
~~~sysml
package ConnectorIncomplete {
    private import StandardViewDefinitions::*;
    part def Assembly {
        port input;
        // The second end names a feature that does not exist, so the connector cannot be drawn
        // and the projection carries a typed relationship-unresolved reason.
        connect input to missingPort;
    }
    view selected : InterconnectionView { expose Assembly; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_connector_incomplete.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 25) (end 6 36))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_connector_incomplete.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 25) (end 6 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:98c77b035a013ff9c8fa0ec05425c27680c00bb4554b6b4eb271d00176d666e2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "input")) (connectorEnd (reference "missingPort")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Assembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "missingPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input"))) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly")))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly")))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (range (start 6 16) (end 6 21)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly::input")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (range (start 6 25) (end 6 36)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "missingPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (range (start 8 20) (end 8 39)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (range (start 8 49) (end 8 57)) (probe (position 8 49))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (path (named (kind package) (name "ConnectorIncomplete")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_connector_incomplete.md") (qualified-name "ConnectorIncomplete::Assembly")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:7fbbb12b3b7cd0bb4f976097716a67fe213df5981bede4d4b57ea3171546a4de",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_interconnection_connector_incomplete.md",
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
        6,
        8,
        6,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        16,
        6,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        25,
        6,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        9,
        8,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ConnectorIncomplete::Assembly"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ConnectorIncomplete::Assembly::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ConnectorIncomplete::Assembly::input"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ConnectorIncomplete::selected"
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
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Ports::ports"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "ConnectorIncomplete::Assembly",
      "source": 2,
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
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "connectorEnd",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "connectorEnd",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typing",
      "source": 1
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
    }
  ],
  "selectedView": {
    "reference": 3,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 5
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
        "navigation": 1,
        "origin": 2,
        "provenance": "authored",
        "reference": 9,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 1,
        "provenance": "authored",
        "reference": 10,
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
        0
      ],
      "ports": [
        2
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              2
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
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 8,
        "source": 2,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "PortUsage",
        "name": "input",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 1,
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
        "reference": 11,
        "source": 0,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 16,
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
        "reference": 17,
        "source": 2,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 18,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 3,
        "provenance": "authored",
        "reference": 12,
        "source": 1,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 4,
        "provenance": "authored",
        "reference": 13,
        "source": 1,
        "target": {
          "status": "unresolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 15,
        "source": 1,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 14,
        "source": 1,
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
