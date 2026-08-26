# META
~~~ini
description=Grid view projects typed rows and relationship columns
type=generate
libraries=standard
plugin=repository:diagram
viewKind=grid-view
viewDocument=diagram_grid_complete.md
viewQualifiedName=GridExample::selected
~~~
# SOURCE
~~~sysml
package GridExample {
    private import StandardViewDefinitions::*;
    part def TableRoot { part first; part second; }
    view selected : GridView { expose TableRoot; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_grid_complete.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 25) (end 2 36))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 37) (end 2 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:c672833c613c1f19ddcf802c690c755bc0cc8f13de9a1632a530e6a0e6152a8a") (contract-version "feature-value-expression-results-v5") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::first"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::second"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GridView")))))
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "TableRoot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GridView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "TableRoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::first"))) (target (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::first"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::second"))) (target (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::second"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::first")))
      (featured-by (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot")))
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
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::second")))
      (featured-by (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot")))
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
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_grid_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_grid_complete.md") (range (start 3 20) (end 3 28)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GridView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")))))
    )
  )
  (query (document "memory://snapshot/diagram_grid_complete.md") (range (start 3 38) (end 3 47)) (probe (position 3 38))
    (reference (id (source (node (document "memory://snapshot/diagram_grid_complete.md") (path (named (kind package) (name "GridExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "TableRoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:21053ef978eb5c92f3a94c4010f1fac21ce120b68019a2313e3128a4f630ff53",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_grid_complete.md",
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
        22
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        30,
        2,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        42,
        2,
        48
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
      "qualifiedName": "GridExample::TableRoot"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GridExample::TableRoot::first"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GridExample::TableRoot::second"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GridExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
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
      "ordinal": 1,
      "relationshipKind": "specializes",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "specializes",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typeFeaturing",
      "source": 2
    }
  ],
  "selectedView": {
    "reference": 3,
    "kind": "grid-view",
    "name": "selected",
    "source": 3
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
        "reference": 6,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 2,
        "provenance": "authored",
        "reference": 7,
        "source": 0,
        "target": 2
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "grid-view",
    "metadata": {
      "cells": [],
      "columns": [
        "specializes",
        "typeFeaturing"
      ],
      "rows": [
        0,
        1,
        2
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              1,
              2
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartDefinition",
        "name": "TableRoot",
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
        "name": "first",
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
        "metaclass": "PartUsage",
        "name": "second",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
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
        "reference": 8,
        "source": 0,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 9,
        "source": 1,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 10,
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
        "reference": 11,
        "source": 2,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 12,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "grid"
    }
  }
}

~~~
