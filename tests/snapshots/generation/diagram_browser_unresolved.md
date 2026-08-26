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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:5efbf8a8e583816d4a27e602c74face2f1d5fbff01a1c05b701e5c6ecfc42860") (admitted (standard-library 94)))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::unrelated"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (path (named (kind package) (name "BrowserNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_browser_unresolved.md") (qualified-name "BrowserNegative::unrelated")))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:5a3f0fe4cbd986b5fe353eba0d29792c75a276cd9c7614701de1536f8584cbe4",
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
    "relationships": [],
    "scene": {
      "kind": "browser"
    }
  }
}

~~~
