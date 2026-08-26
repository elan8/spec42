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
    part def Assembly { port input; port output; part nested; }
    view selected : InterconnectionView { expose Assembly; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_interconnection_complete.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 2 24) (end 2 35))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 2 36) (end 2 48))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 49) (end 2 61))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:d0899b33fd5aad9861a5d41b20e7ced9e208309d540c35a4c20ef2c870283704") (contract-version "constructor-expression-result-v8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Assembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (path (named (kind package) (name "InterconnectionExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (target (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::input")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::nested")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly::output")))
      (featured-by (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
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
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 3 20) (end 3 39)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_interconnection_complete.md") (qualified-name "InterconnectionExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/diagram_interconnection_complete.md") (range (start 3 49) (end 3 57)) (probe (position 3 49))
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
  "modelDigest": "blake3:a26f8f2d95f03b769cc7cb218c54a4a5a3e7fcf771104e230ccc322b02163190",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_interconnection_complete.md",
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
        2,
        13,
        2,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        29,
        2,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        41,
        2,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        54,
        2,
        60
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        9,
        3,
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
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part::ownedPorts"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
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
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "specializes",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "specializes",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "specializes",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "specializes",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "specializes",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 3
    }
  ],
  "selectedView": {
    "reference": 4,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 4
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
        "provenance": "authored",
        "reference": 9,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 3,
        "provenance": "authored",
        "reference": 10,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 2,
        "provenance": "authored",
        "reference": 11,
        "source": 0,
        "target": 3
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [],
      "parts": [
        0,
        1
      ],
      "ports": [
        2,
        3
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              1
            ],
            "provenance": "direct"
          },
          {
            "kind": "ports",
            "members": [
              2,
              3
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
        "metaclass": "PartUsage",
        "name": "nested",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 3,
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
        "reference": 1,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "PortUsage",
        "name": "output",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
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
        "reference": 12,
        "source": 0,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 13,
        "source": 2,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 14,
        "source": 2,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 15,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 16,
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
        "reference": 17,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 18,
        "source": 3,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 19,
        "source": 3,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 20,
        "source": 3,
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
