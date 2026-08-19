# META
~~~ini
description=Action flow view projects actions and authored succession
type=generate
libraries=standard
plugin=repository:diagram
viewKind=action-flow-view
viewDocument=diagram_action_flow_complete.md
viewQualifiedName=ActionFlowExample::selected
~~~
# SOURCE
~~~sysml
package ActionFlowExample {
    private import StandardViewDefinitions::*;
    action def Process { action prepare; action execute; first prepare then execute; }
    view selected : ActionFlowView { expose Process; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_action_flow_complete.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:3197fb9ed16ea19bebd6b9e85bccc209e55479c489399f91032fffacf61b00d2") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "prepare")) (succession (reference "execute")))))
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::execute"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::prepare"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Process")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "prepare")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::prepare")))))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "execute")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::execute")))))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::prepare"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::execute"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::execute")))
      (featured-by (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::prepare")))
      (featured-by (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_action_flow_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_action_flow_complete.md") (range (start 2 63) (end 2 70)) (probe (position 2 63))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "prepare")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::prepare")))))
    )
  )
  (query (document "memory://snapshot/diagram_action_flow_complete.md") (range (start 2 76) (end 2 83)) (probe (position 2 76))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "execute")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process::execute")))))
    )
  )
  (query (document "memory://snapshot/diagram_action_flow_complete.md") (range (start 3 20) (end 3 34)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/diagram_action_flow_complete.md") (range (start 3 44) (end 3 51)) (probe (position 3 44))
    (reference (id (source (node (document "memory://snapshot/diagram_action_flow_complete.md") (path (named (kind package) (name "ActionFlowExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_action_flow_complete.md") (qualified-name "ActionFlowExample::Process")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 2,
  "modelDigest": "blake3:ae89f0c0e7c17cba708db9becb3ff301a7c7e0d9818ecb4a9b1ff2ebb0e7faea",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_action_flow_complete.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        2,
        15,
        2,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        32,
        2,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        48,
        2,
        55
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        57,
        2,
        84
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        63,
        2,
        70
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        76,
        2,
        83
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
      "qualifiedName": "ActionFlowExample::Process"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ActionFlowExample::Process::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ActionFlowExample::Process::execute"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ActionFlowExample::Process::prepare"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ActionFlowExample::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "ActionFlowExample::Process",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "succession",
      "source": 3
    }
  ],
  "selectedView": {
    "reference": 4,
    "kind": "action-flow-view",
    "name": "selected",
    "source": 6
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
        "navigation": 3,
        "provenance": "authored",
        "reference": 6,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 4,
        "provenance": "authored",
        "reference": 11,
        "source": 3,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 2,
        "provenance": "authored",
        "reference": 7,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 1,
        "provenance": "authored",
        "reference": 8,
        "source": 0,
        "target": 3
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "action-flow-view",
    "metadata": {
      "actions": [
        0,
        2,
        3
      ],
      "controlNodes": []
    },
    "nodes": [
      {
        "metaclass": "ActionDefinition",
        "name": "Process",
        "owner": null,
        "reference": 0,
        "source": 0
      },
      {
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "owner": 0,
        "reference": 5,
        "source": 3
      },
      {
        "metaclass": "ActionUsage",
        "name": "execute",
        "owner": 0,
        "reference": 2,
        "source": 2
      },
      {
        "metaclass": "ActionUsage",
        "name": "prepare",
        "owner": 0,
        "reference": 3,
        "source": 1
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 4,
        "provenance": "authored",
        "reference": 9,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 5,
        "provenance": "authored",
        "reference": 10,
        "source": 1,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      }
    ]
  }
}

~~~
