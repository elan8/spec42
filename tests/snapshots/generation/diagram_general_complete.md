# META
~~~ini
description=General view projects exposed ownership and published relationships
type=generate
libraries=standard
plugin=repository:diagram
viewKind=general-view
viewDocument=diagram_general_complete.md
viewQualifiedName=GeneralExample::selected
~~~
# SOURCE
~~~sysml
package GeneralExample {
    private import StandardViewDefinitions::*;
    part def System { part child; }
    view selected : GeneralView { expose System; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_general_complete.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 22) (end 2 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:6da8af2d04690d04e772b3effe5ca812be4aec594772d643dc1ac22649f81bbc") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::System::child"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "System")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::System")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::System::child")))
      (featured-by (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::System")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_general_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_complete.md") (range (start 3 20) (end 3 31)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_complete.md") (range (start 3 41) (end 3 47)) (probe (position 3 41))
    (reference (id (source (node (document "memory://snapshot/diagram_general_complete.md") (path (named (kind package) (name "GeneralExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_complete.md") (qualified-name "GeneralExample::System")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 2,
  "modelDigest": "blake3:25aa361629a136c7bb2c0a6c3b6f21da0f91ff8f1b33bd68ae74f81ed59b59cf",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_general_complete.md",
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
        19
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        27,
        2,
        32
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
      "qualifiedName": "GeneralExample::System"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralExample::System::child"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralExample::selected"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 0
    }
  ],
  "selectedView": {
    "reference": 2,
    "kind": "general-view",
    "name": "selected",
    "source": 2
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
        "reference": 3,
        "source": 0,
        "target": 1
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "general-view",
    "metadata": {
      "roots": [
        0
      ]
    },
    "nodes": [
      {
        "metaclass": "PartDefinition",
        "name": "System",
        "owner": null,
        "reference": 0,
        "source": 0
      },
      {
        "metaclass": "PartUsage",
        "name": "child",
        "owner": 0,
        "reference": 1,
        "source": 1
      }
    ],
    "relationships": []
  }
}

~~~
