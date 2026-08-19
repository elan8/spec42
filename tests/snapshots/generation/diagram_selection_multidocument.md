# META
~~~ini
description=Qualified selection chooses one of several views distributed across documents
type=generate
libraries=standard
plugin=repository:diagram
viewKind=general-view
viewDocument=structure.sysml
viewQualifiedName=StructureModel::selected
~~~
# SOURCE
## structure.sysml
~~~sysml
package StructureModel {
    private import StandardViewDefinitions::*;
    part def SelectedRoot { part included; }
    view selected : GeneralView { expose SelectedRoot; }
}
~~~
## behavior.sysml
~~~sysml
package BehaviorModel {
    private import StandardViewDefinitions::*;
    action def UnrelatedAction { action hidden; }
    view other : ActionFlowView { expose UnrelatedAction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/behavior.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/structure.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 28) (end 2 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:cd1feb467dac01a3a89256904ace4c47dc547fca71b7ffa6b761410fbce02b28") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::UnrelatedAction"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::UnrelatedAction::hidden"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::other"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (named (kind view) (name "other")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "UnrelatedAction")))))
    (declaration (id (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::SelectedRoot"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::SelectedRoot::included"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "SelectedRoot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (named (kind view) (name "other")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "UnrelatedAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::UnrelatedAction")))))
    (reference (id (source (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "SelectedRoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::SelectedRoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::other"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (named (kind view) (name "other")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::UnrelatedAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (named (kind view) (name "other")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::SelectedRoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::UnrelatedAction::hidden")))
      (featured-by (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::UnrelatedAction")))
    )
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::other")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (named (kind view) (name "other")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::other")))
    )
    (declaration (id (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::SelectedRoot::included")))
      (featured-by (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::SelectedRoot")))
    )
    (declaration (id (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/behavior.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/behavior.sysml") (range (start 3 17) (end 3 31)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::other"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/behavior.sysml") (range (start 3 41) (end 3 56)) (probe (position 3 41))
    (reference (id (source (node (document "memory://snapshot/behavior.sysml") (path (named (kind package) (name "BehaviorModel")) (named (kind view) (name "other")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "UnrelatedAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior.sysml") (qualified-name "BehaviorModel::UnrelatedAction")))))
    )
  )
  (query (document "memory://snapshot/structure.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/structure.sysml") (range (start 3 20) (end 3 31)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/structure.sysml") (range (start 3 41) (end 3 53)) (probe (position 3 41))
    (reference (id (source (node (document "memory://snapshot/structure.sysml") (path (named (kind package) (name "StructureModel")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "SelectedRoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/structure.sysml") (qualified-name "StructureModel::SelectedRoot")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 4,
  "modelDigest": "blake3:60d6bb52f861c8517b7fc77f9acae7cbb27843f00869f431fa6d7e9044abcff8",
  "documents": [
    {
      "uri": "memory://snapshot/structure.sysml",
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
        25
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        33,
        2,
        41
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
      "qualifiedName": "StructureModel::SelectedRoot"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StructureModel::SelectedRoot::included"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StructureModel::selected"
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
        "compartments": [
          {
            "kind": "parts",
            "members": [
              1
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartDefinition",
        "name": "SelectedRoot",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 0
      },
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "included",
        "notationRole": "usage",
        "owner": 0,
        "reference": 1,
        "source": 1
      }
    ],
    "relationships": [],
    "scene": {
      "kind": "general"
    }
  }
}

~~~
