# META
~~~ini
description=Geometry view preserves exposed elements and reports absent geometry facts
type=generate
libraries=standard
plugin=repository:diagram
viewKind=geometry-view
viewDocument=diagram_geometry_incomplete.md
viewQualifiedName=GeometryExample::selected
~~~
# SOURCE
~~~sysml
package GeometryExample {
    private import StandardViewDefinitions::*;
    part def Shape;
    view selected : GeometryView { expose Shape; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_geometry_incomplete.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:cd1533e0f705a7b2c58dee234aa3cf4a76e04be85c15ceb525c4fe29d47ea5f8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::Shape"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeometryView")))))
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Shape")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeometryView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Shape")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::Shape")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::Shape"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::Shape"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::Shape")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_geometry_incomplete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_geometry_incomplete.md") (range (start 3 20) (end 3 32)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeometryView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")))))
    )
  )
  (query (document "memory://snapshot/diagram_geometry_incomplete.md") (range (start 3 42) (end 3 47)) (probe (position 3 42))
    (reference (id (source (node (document "memory://snapshot/diagram_geometry_incomplete.md") (path (named (kind package) (name "GeometryExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Shape")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_geometry_incomplete.md") (qualified-name "GeometryExample::Shape")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:ef51566d3f12647b7014f2f2cdb5832320fe380b6f82c5d8ad54cd6a6258aae0",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_geometry_incomplete.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
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
        18
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
      "qualifiedName": "GeometryExample::Shape"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeometryExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    }
  ],
  "selectedView": {
    "reference": 1,
    "kind": "geometry-view",
    "name": "selected",
    "source": 1
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "geometry-facts-unavailable"
      }
    ]
  },
  "projection": {
    "edges": [],
    "exposedRoots": [
      0
    ],
    "kind": "geometry-view",
    "metadata": {
      "elements": [
        0
      ],
      "primitives": []
    },
    "nodes": [
      {
        "compartments": [],
        "metaclass": "PartDefinition",
        "name": "Shape",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 0,
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
        "reference": 3,
        "source": 0,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "geometry"
    }
  }
}

~~~
