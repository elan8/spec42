# META
~~~ini
description=Sequence limitations retain unsupported library constructs even when library diagnostics are outside the reporting scope
type=generate
libraries=none
standard_library_document=library.sysml
plugin=native:diagram
viewKind=sequence-view
viewDocument=model.sysml
viewQualifiedName=Model::selected
~~~
# SOURCE
## library.sysml
~~~sysml
package StandardViewDefinitions { view def SequenceView; }
package Library {
    occurrence def Scenario {
        part left { event occurrence sent; }
        part right { event occurrence received; }
        message exchange from left.sent to right.received;
    }
    state def Machine { constraint pending { true } }
}
~~~
## model.sysml
~~~sysml
package Model {
    view selected : StandardViewDefinitions::SequenceView { expose Library::Scenario; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 1 4) (end 1 87))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 1 4) (end 1 87))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:215cf40f173b57088db6fdaebccdc5b405243f66005d9d7021ef6288d836a50d") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::SequenceView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Library::Scenario")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/library.sysml") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Library::Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/library.sysml") (qualified-name "Library::Scenario")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (target (node (document "memory://snapshot/library.sysml") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/library.sysml") (qualified-name "Library::Scenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected")))
      (type (node (document "memory://snapshot/library.sysml") (qualified-name "StandardViewDefinitions::SequenceView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/library.sysml") (qualified-name "StandardViewDefinitions::SequenceView")) (source direct))
      (supertype (node (document "memory://snapshot/library.sysml") (qualified-name "StandardViewDefinitions::SequenceView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 20) (end 1 57)) (probe (position 1 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/library.sysml") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 1 67) (end 1 84)) (probe (position 1 67))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Library::Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/library.sysml") (qualified-name "Library::Scenario")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 12,
  "modelDigest": "blake3:42e45ace0caa507331fb6e32fd409a35045dbd57fce74c6f32d9b22ee2bf3aaa",
  "documents": [
    {
      "uri": "memory://snapshot/library.sysml",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        2,
        19,
        2,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        24,
        7,
        51
      ]
    },
    {
      "document": 1,
      "range": [
        1,
        9,
        1,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Model::selected"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Library::Scenario"
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "sequence-view",
    "name": "selected",
    "source": 2
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "unsupported-syntax",
        "diagnostics": [
          {
            "code": "unsupported_state_definition_member",
            "message": "This member is parsed but not modelled by the semantic publication.",
            "source": 1
          }
        ]
      }
    ]
  },
  "projection": {
    "edges": [],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [],
      "participants": []
    },
    "nodes": [
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "OccurrenceDefinition",
        "multiplicity": null,
        "name": "Scenario",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:a1e79e05eb8854ed09cc79f9fb7cf78030583208fabc9008840b92c3d24510a4",
        "reference": 1,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [],
    "scene": {
      "kind": "sequence",
      "lifelines": [],
      "messages": []
    }
  }
}

~~~
