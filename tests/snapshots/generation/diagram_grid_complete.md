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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:c672833c613c1f19ddcf802c690c755bc0cc8f13de9a1632a530e6a0e6152a8a") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
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
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::first")))
      (featured-by (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot::second")))
      (featured-by (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::TableRoot")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_grid_complete.md") (qualified-name "GridExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (scopes any))
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
  "schemaVersion": 2,
  "modelDigest": "blake3:b780a516c8bc22ae0f1c0c53d80bdaeb6383ddad5136e3b52617f60c1a3023ca",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_grid_complete.md",
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
        "reference": 4,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 2,
        "provenance": "authored",
        "reference": 5,
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
      "columns": [],
      "rows": [
        0,
        1,
        2
      ]
    },
    "nodes": [
      {
        "metaclass": "PartDefinition",
        "name": "TableRoot",
        "owner": null,
        "reference": 0,
        "source": 0
      },
      {
        "metaclass": "PartUsage",
        "name": "first",
        "owner": 0,
        "reference": 1,
        "source": 1
      },
      {
        "metaclass": "PartUsage",
        "name": "second",
        "owner": 0,
        "reference": 2,
        "source": 2
      }
    ],
    "relationships": []
  }
}

~~~
