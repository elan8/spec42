# META
~~~ini
description=An authored view with no exposure produces a valid empty projection
type=generate
libraries=standard
plugin=repository:diagram
viewKind=browser-view
viewDocument=diagram_empty_valid.md
viewQualifiedName=EmptyExample::selected
~~~
# SOURCE
~~~sysml
package EmptyExample {
    private import StandardViewDefinitions::*;
    part unrelated;
    view selected : BrowserView;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_empty_valid.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 4) (end 2 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:40ad01970860672f8d247c2587669562e3e2f2031840e2693978148f41168a99") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_empty_valid.md") (path (named (kind package) (name "EmptyExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BrowserView")))))
    (declaration (id (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample::unrelated"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_empty_valid.md") (path (named (kind package) (name "EmptyExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample::selected"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_empty_valid.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_empty_valid.md") (path (named (kind package) (name "EmptyExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_empty_valid.md") (range (start 3 20) (end 3 31)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_empty_valid.md") (qualified-name "EmptyExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:fae636f11b8700dcc3235d4a6b24f9e5c0841d73d3fad1937ee88ec48f111e94",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_empty_valid.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
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
      "qualifiedName": "EmptyExample::selected"
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "browser-view",
    "name": "selected",
    "source": 0
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [],
    "exposedRoots": [],
    "kind": "browser-view",
    "metadata": {
      "roots": []
    },
    "nodes": [],
    "relationships": [],
    "scene": {
      "kind": "browser"
    }
  }
}

~~~
