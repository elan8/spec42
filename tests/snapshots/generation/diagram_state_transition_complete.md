# META
~~~ini
description=State transition view projects states initial final and transitions
type=generate
libraries=standard
plugin=repository:diagram
viewKind=state-transition-view
viewDocument=diagram_state_transition_complete.md
viewQualifiedName=StateExample::selected
~~~
# SOURCE
~~~sysml
package StateExample {
    private import StandardViewDefinitions::*;
    item def Start;
    state def Machine {
        then idle;
        state idle;
        final done;
        transition finish first idle accept Start then done;
    }
    view selected : StateTransitionView { expose Machine; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_state_transition_complete.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:aab42d851e2e58d57f22ee31c4d1530c5657248959ede0fdc4f57c110ca50024") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "done")) (transitionTrigger (reference "Start")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Machine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 4 13) (end 4 17)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 7 32) (end 7 36)) (probe (position 7 32))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 7 55) (end 7 59)) (probe (position 7 55))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 7 44) (end 7 49)) (probe (position 7 44))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0) (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 9 20) (end 9 39)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 9 49) (end 9 56)) (probe (position 9 49))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 2,
  "modelDigest": "blake3:a4f484005023617686514807b29c4c7620cbdb76bb3db5383c36203c01f871b9",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_state_transition_complete.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        14,
        3,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        8,
        4,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        14,
        5,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        14,
        6,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        19,
        7,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        32,
        7,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        44,
        7,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        55,
        7,
        59
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        9,
        9,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::done"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::finish"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Start"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "StateExample::Machine",
      "source": 1,
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
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "initialState",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "initialState",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "transitionSource",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "transitionTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "transitionTrigger",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "transition",
      "source": 4
    }
  ],
  "selectedView": {
    "reference": 6,
    "kind": "state-transition-view",
    "name": "selected",
    "source": 9
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 5,
        "provenance": "authored",
        "reference": 8,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 6,
        "provenance": "authored",
        "reference": 17,
        "source": 4,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 4,
        "provenance": "authored",
        "reference": 9,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 1,
        "provenance": "authored",
        "reference": 10,
        "source": 0,
        "target": 3
      },
      {
        "kind": "initial-state",
        "navigation": 2,
        "provenance": "authored",
        "reference": 13,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 3,
        "provenance": "authored",
        "reference": 11,
        "source": 0,
        "target": 4
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [
        2
      ],
      "initialNodes": [
        3
      ],
      "states": [
        0,
        4
      ]
    },
    "nodes": [
      {
        "metaclass": "StateDefinition",
        "name": "Machine",
        "owner": null,
        "reference": 0,
        "source": 0
      },
      {
        "metaclass": "TransitionUsage",
        "name": "finish",
        "owner": 0,
        "reference": 3,
        "source": 5
      },
      {
        "metaclass": "FinalState",
        "name": "done",
        "owner": 0,
        "reference": 2,
        "source": 4
      },
      {
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "owner": 0,
        "reference": 7,
        "source": 1
      },
      {
        "metaclass": "StateUsage",
        "name": "idle",
        "owner": 0,
        "reference": 4,
        "source": 3
      }
    ],
    "relationships": [
      {
        "kind": "transitionSource",
        "navigation": 6,
        "provenance": "authored",
        "reference": 14,
        "source": 1,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 8,
        "provenance": "authored",
        "reference": 15,
        "source": 1,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 7,
        "provenance": "authored",
        "reference": 16,
        "source": 1,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 2,
        "provenance": "authored",
        "reference": 12,
        "source": 3,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      }
    ]
  }
}

~~~
