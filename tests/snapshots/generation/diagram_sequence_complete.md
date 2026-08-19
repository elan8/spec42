# META
~~~ini
description=Sequence view projects authoritative participants and flow facts
type=generate
libraries=standard
plugin=repository:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_complete.md
viewQualifiedName=SequenceExample::selected
~~~
# SOURCE
~~~sysml
package SequenceExample {
    private import StandardViewDefinitions::*;
    part def Interaction { part sender; part receiver; }
    view selected : SequenceView { expose Interaction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_complete.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 27) (end 2 39))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 40) (end 2 54))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:ef97e0a46425217f5384ee88c32bbbbdff557b35619ae5a8d01dfa961ac5a0c1") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::receiver"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::sender"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Interaction")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::receiver")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::sender")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 3 20) (end 3 32)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 3 42) (end 3 53)) (probe (position 3 42))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 2,
  "modelDigest": "blake3:5275e8bc3437a500f18e59ec95363069309c2bc3542a6c057d6a49376bcb767a",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_complete.md",
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
        24
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        32,
        2,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        45,
        2,
        53
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
      "qualifiedName": "SequenceExample::Interaction"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::receiver"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::sender"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::selected"
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
    "kind": "sequence-view",
    "name": "selected",
    "source": 3
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "parse-recovery"
      }
    ]
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
    "kind": "sequence-view",
    "metadata": {
      "messages": [],
      "participants": [
        1,
        2
      ]
    },
    "nodes": [
      {
        "metaclass": "PartDefinition",
        "name": "Interaction",
        "owner": null,
        "reference": 0,
        "source": 0
      },
      {
        "metaclass": "PartUsage",
        "name": "sender",
        "owner": 0,
        "reference": 2,
        "source": 1
      },
      {
        "metaclass": "PartUsage",
        "name": "receiver",
        "owner": 0,
        "reference": 1,
        "source": 2
      }
    ],
    "relationships": []
  }
}

~~~
