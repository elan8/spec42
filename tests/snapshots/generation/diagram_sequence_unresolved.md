# META
~~~ini
description=Sequence view keeps unresolved exposure explicit
type=generate
libraries=standard
plugin=repository:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_unresolved.md
viewQualifiedName=SequenceNegative::selected
~~~
# SOURCE
~~~sysml
package SequenceNegative {
    private import StandardViewDefinitions::*;
    part unrelated;
    view selected : SequenceView { expose Missing; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_unresolved.md"
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
        (range (start 3 42) (end 3 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:eaed36cecf940a559efe2d04a551b1fb5ca6dface7d64a8d171729a1f39f9c4f") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved.md") (path (named (kind package) (name "SequenceNegative")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved.md") (path (named (kind package) (name "SequenceNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Missing")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::unrelated"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (path (named (kind package) (name "SequenceNegative")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (path (named (kind package) (name "SequenceNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::selected"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved.md") (path (named (kind package) (name "SequenceNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_unresolved.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (path (named (kind package) (name "SequenceNegative")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_unresolved.md") (range (start 3 20) (end 3 32)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (qualified-name "SequenceNegative::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_unresolved.md") (range (start 3 42) (end 3 49)) (probe (position 3 42))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved.md") (path (named (kind package) (name "SequenceNegative")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Missing")
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
  "modelDigest": "blake3:ca82f2f96ab43423439b76a645f6645990cb3d0d836bfbdacc7344a40ddf4b89",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_unresolved.md",
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
        35,
        3,
        50
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceNegative::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Expose",
      "ownerQualifiedName": "SequenceNegative::selected",
      "source": 1,
      "sourceDomain": "workspace"
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "sequence-view",
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
    "kind": "sequence-view",
    "metadata": {
      "messages": [],
      "participants": []
    },
    "nodes": [],
    "relationships": []
  }
}

~~~
