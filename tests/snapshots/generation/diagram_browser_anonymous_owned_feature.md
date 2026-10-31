# META
~~~ini
description=Browser retains an authored anonymous KerML feature
type=generate
libraries=standard
plugin=native:diagram
viewKind=browser-view
viewDocument=views.sysml
viewQualifiedName=BrowserProbe::overview
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
## structure.kerml
~~~kerml
package KernelModel {
    class Thing;
    class Holder {
        feature : Thing;
    }
}
~~~
## views.sysml
~~~sysml
package BrowserProbe {
    view overview : StandardViewDefinitions::BrowserView { expose KernelModel::Holder; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/structure.kerml"
    (diagnostics
    )
  )
  (document "memory://snapshot/views.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:638aa0c870ecf7db2ff7b04d8869f338f1e44c4ef124c60f9e98fbca3e6e63dc") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::BrowserView")))))
    (declaration (id (node (document "memory://snapshot/views.sysml") (path (named (kind package) (name "BrowserProbe")) (named (kind view) (name "overview")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "KernelModel::Holder")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing")))))
    (reference (id (source (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    (reference (id (source (node (document "memory://snapshot/views.sysml") (path (named (kind package) (name "BrowserProbe")) (named (kind view) (name "overview")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "KernelModel::Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/views.sysml") (path (named (kind package) (name "BrowserProbe")) (named (kind view) (name "overview")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.sysml") (path (named (kind package) (name "BrowserProbe")) (named (kind view) (name "overview")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.sysml") (path (named (kind package) (name "BrowserProbe")) (named (kind view) (name "overview")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder")))
      (type (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.sysml") (path (named (kind package) (name "BrowserProbe")) (named (kind view) (name "overview")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/structure.kerml") (range (start 3 18) (end 3 23)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/structure.kerml") (path (named (kind package) (name "KernelModel")) (named (kind class-def) (name "Holder")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Thing")))))
    )
  )
  (query (document "memory://snapshot/views.sysml") (range (start 1 20) (end 1 56)) (probe (position 1 20))
    (reference (id (source (node (document "memory://snapshot/views.sysml") (qualified-name "BrowserProbe::overview"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    )
  )
  (query (document "memory://snapshot/views.sysml") (range (start 1 66) (end 1 85)) (probe (position 1 66))
    (reference (id (source (node (document "memory://snapshot/views.sysml") (path (named (kind package) (name "BrowserProbe")) (named (kind view) (name "overview")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "KernelModel::Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/structure.kerml") (qualified-name "KernelModel::Holder")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:3447d0bfb87a3fa2dd1c5b2d73ed86210be4ff5777c5e77d5f7e98e82862f126",
  "documents": [
    {
      "uri": "memory://snapshot/structure.kerml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/views.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        2,
        10,
        2,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        8,
        3,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        18,
        3,
        23
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
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KernelModel::Holder"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KernelModel::Holder::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KernelModel::Thing"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "BrowserProbe::overview"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "KernelModel::Holder",
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
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 1
    }
  ],
  "selectedView": {
    "reference": 3,
    "kind": "browser-view",
    "name": "overview",
    "source": 3
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
        "origin": 1,
        "provenance": "authored",
        "reference": 5,
        "source": 0,
        "target": 1
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "browser-view",
    "metadata": {
      "roots": [
        0
      ],
      "rows": [
        {
          "key": "96:element/v133:memory://snapshot/structure.kerml7:packagen11:KernelModel1:09:class-defn6:Holder1:0",
          "members": "loaded",
          "membership": "owned",
          "node": 0
        },
        {
          "key": "96:element/v133:memory://snapshot/structure.kerml7:packagen11:KernelModel1:09:class-defn6:Holder1:0116:element/v133:memory://snapshot/structure.kerml7:packagen11:KernelModel1:09:class-defn6:Holder1:013:kerml-featurea1:0",
          "members": "none",
          "membership": "owned",
          "node": 1
        }
      ]
    },
    "nodes": [
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "Class",
        "multiplicity": null,
        "name": "Holder",
        "notationRole": "unsupported",
        "owner": null,
        "presentationKey": "p:f11e16283f9313cd8a04594aad51b83111aa5b15cb479d4af6776745c3f6d3b9",
        "reference": 0,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:c629a3298aa9a7f15d55321f343de1ebef1be2aa8c87ca7bb643b9ca587b67d7",
        "reference": 4,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Thing",
              "reference": 2
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 6,
        "source": 1,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "browser"
    }
  }
}

~~~
