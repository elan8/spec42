# META
~~~ini
description=Browser view projects canonical membership tree
type=generate
libraries=standard
plugin=repository:diagram
viewKind=browser-view
viewDocument=diagram_browser_complete.md
viewQualifiedName=BrowserExample::selected
~~~
# SOURCE
~~~sysml
package BrowserExample {
    private import StandardViewDefinitions::*;
    part def Root { part branch { part leaf; } }
    view selected : BrowserView { expose Root; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_browser_complete.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 20) (end 2 46))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 34) (end 2 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:b72dc185f0f7dd98f118430fa8e03ce00bfa4a26ae0233813f17c46bb12047a2") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root::branch"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root::branch::leaf"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BrowserView")))))
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Root")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Root")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root::branch")))
      (featured-by (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root::branch::leaf")))
      (featured-by (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root::branch")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_browser_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_browser_complete.md") (range (start 3 20) (end 3 31)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    )
  )
  (query (document "memory://snapshot/diagram_browser_complete.md") (range (start 3 41) (end 3 45)) (probe (position 3 41))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_complete.md") (path (named (kind package) (name "BrowserExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Root")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_browser_complete.md") (qualified-name "BrowserExample::Root")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 2,
  "modelDigest": "blake3:da3b01538454be5bdb18db4056b8af8b78e50376b730804d30a832a52cadddbc",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_browser_complete.md",
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
        17
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        25,
        2,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        39,
        2,
        43
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
      "qualifiedName": "BrowserExample::Root"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "BrowserExample::Root::branch"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "BrowserExample::Root::branch::leaf"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "BrowserExample::selected"
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
      "source": 1
    }
  ],
  "selectedView": {
    "reference": 3,
    "kind": "browser-view",
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
        "source": 1,
        "target": 2
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "browser-view",
    "metadata": {
      "roots": [
        0
      ]
    },
    "nodes": [
      {
        "metaclass": "PartDefinition",
        "name": "Root",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 0
      },
      {
        "metaclass": "PartUsage",
        "name": "branch",
        "notationRole": "usage",
        "owner": 0,
        "reference": 1,
        "source": 1
      },
      {
        "metaclass": "PartUsage",
        "name": "leaf",
        "notationRole": "usage",
        "owner": 1,
        "reference": 2,
        "source": 2
      }
    ],
    "relationships": []
  }
}

~~~
