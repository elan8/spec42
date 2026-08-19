# META
~~~ini
description=Browser view keeps unresolved exposure explicit
type=generate
libraries=standard
plugin=repository:diagram
viewKind=browser-view
viewDocument=diagram_browser_unresolved.md
viewQualifiedName=BrowserNegative::selected
~~~
# SOURCE
~~~sysml
package BrowserNegative {
    private import StandardViewDefinitions::*;
    part unrelated;
    view selected : BrowserView { expose Missing; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_browser_unresolved.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 4) (end 2 19))
      )
      (diagnostic
        (severity warning)
        (code "view_expose_unresolved")
        (source "semantic")
        (range (start 3 41) (end 3 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:5efbf8a8e583816d4a27e602c74face2f1d5fbff01a1c05b701e5c6ecfc42860") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BrowserView")))))
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Missing")))))
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::unrelated"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_browser_unresolved.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_browser_unresolved.md") (range (start 3 20) (end 3 31)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected"))) (kind featureTyping) (ordinal 0) (authored-target "BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    )
  )
  (query (document "memory://snapshot/diagram_browser_unresolved.md") (range (start 3 41) (end 3 48)) (probe (position 3 41))
    (reference (id (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 2,
  "modelDigest": "blake3:a71e4bdbf2bde9ab77565dfa649a8be3b069694ceb569d4654135bbeec2a0572",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_browser_unresolved.md",
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
    },
    {
      "document": 0,
      "range": [
        3,
        34,
        3,
        49
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "BrowserNegative::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Expose",
      "ownerQualifiedName": "BrowserNegative::selected",
      "source": 1,
      "sourceDomain": "workspace"
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "browser-view",
    "name": "selected",
    "source": 0
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "parse-recovery"
      },
      {
        "code": "exposure-unresolved",
        "exposure": 1
      }
    ]
  },
  "projection": {
    "edges": [],
    "exposedRoots": [],
    "kind": "browser-view",
    "metadata": {
      "roots": []
    },
    "nodes": [],
    "relationships": []
  }
}

~~~
