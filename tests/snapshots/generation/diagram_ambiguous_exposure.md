# META
~~~ini
description=Ambiguous exposure remains explicit and admits no guessed scope
type=generate
libraries=standard
plugin=repository:diagram
viewKind=general-view
viewDocument=diagram_ambiguous_exposure.md
viewQualifiedName=AmbiguousExample::selected
~~~
# SOURCE
~~~sysml
package AmbiguousExample {
    private import StandardViewDefinitions::*;
    part def Duplicate;
    part def Duplicate;
    view selected : GeneralView { expose Duplicate; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_ambiguous_exposure.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 3 4) (end 3 23))
        (related-information
          (related
            (uri "memory://snapshot/diagram_ambiguous_exposure.md")
            (range (start 2 4) (end 2 23))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 4 41) (end 4 50))
        (related-information
          (related
            (uri "memory://snapshot/diagram_ambiguous_exposure.md")
            (range (start 2 4) (end 2 23))
          )
          (related
            (uri "memory://snapshot/diagram_ambiguous_exposure.md")
            (range (start 3 4) (end 3 23))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:f14a5a180f45e68462fafa495c434bc62cb11e6b2856e4c9dd8bdbb64f575100") (contract-version "operator-expression-arguments-v7") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate"))))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate") (occurrence 1))))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Duplicate")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Duplicate")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate")))) (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate") (occurrence 1)))))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate"))))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate") (occurrence 1))))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate")))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate") (occurrence 1)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_ambiguous_exposure.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_ambiguous_exposure.md") (range (start 4 20) (end 4 31)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (qualified-name "AmbiguousExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_ambiguous_exposure.md") (range (start 4 41) (end 4 50)) (probe (position 4 41))
    (reference (id (source (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Duplicate")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate")))) (node (document "memory://snapshot/diagram_ambiguous_exposure.md") (path (named (kind package) (name "AmbiguousExample")) (named (kind part-def) (name "Duplicate") (occurrence 1)))))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:9753827b5acbaf0feb4be0ddb75a0c40768e01269d5ebe10ae24db9f5866416c",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_ambiguous_exposure.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        4,
        9,
        4,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        34,
        4,
        51
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "AmbiguousExample::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Expose",
      "ownerQualifiedName": "AmbiguousExample::selected",
      "source": 1,
      "sourceDomain": "workspace"
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "general-view",
    "name": "selected",
    "source": 0
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "exposure-ambiguous",
        "exposure": 1
      }
    ]
  },
  "projection": {
    "edges": [],
    "exposedRoots": [],
    "kind": "general-view",
    "metadata": {
      "roots": []
    },
    "nodes": [],
    "relationships": [],
    "scene": {
      "kind": "general"
    }
  }
}

~~~
