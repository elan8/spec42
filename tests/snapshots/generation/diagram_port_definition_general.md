# META
~~~ini
description=Issue 284: GeneralView exposes a port definition and retains implicit conjugation without panicking
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=diagram_port_definition_general.md
viewQualifiedName=M::g
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package M {
    port def P;
    view g : StandardViewDefinitions::GeneralView { expose M::P; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_port_definition_general.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a66ab910a77e1acc6c84515787788ab44f2cd162373316ee427e7fb199f54ac7") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::P"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~P") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "M::P")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "M::P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::P"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::P"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_port_definition_general.md") (range (start 2 13) (end 2 49)) (probe (position 2 13))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_port_definition_general.md") (range (start 2 59) (end 2 63)) (probe (position 2 59))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_general.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "M::P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_port_definition_general.md") (qualified-name "M::P")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:3c0002c64189847bfb63469c9076385f45b97d1207c84dfd84c90ef8df343593",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_port_definition_general.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/ports.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        1,
        4,
        1,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        1,
        13,
        1,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        9,
        2,
        10
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "M::P"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "M::P::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "M::g"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Ports::Port"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConjugatedPortDefinition",
      "ownerQualifiedName": "M::P",
      "source": 0,
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
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "conjugation",
      "source": 1
    }
  ],
  "selectedView": {
    "reference": 2,
    "kind": "general-view",
    "name": "g",
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
        "navigation": 0,
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
    "kind": "general-view",
    "metadata": {
      "roots": [
        0
      ]
    },
    "nodes": [
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortDefinition",
        "name": "P",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 1,
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
        "metaclass": "ConjugatedPortDefinition",
        "name": null,
        "notationRole": "definition",
        "owner": 0,
        "reference": 4,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 6,
        "source": 0,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "conjugation",
        "navigation": null,
        "provenance": "implied",
        "reference": 7,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "general"
    }
  }
}

~~~
