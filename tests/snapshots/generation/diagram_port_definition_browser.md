# META
~~~ini
description=Issue 284: BrowserView exposes a port definition and retains implicit conjugation without panicking
type=generate
libraries=standard
plugin=native:diagram
viewKind=browser-view
viewDocument=diagram_port_definition_browser.md
viewQualifiedName=M::g
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package M {
    port def P;
    view g : StandardViewDefinitions::BrowserView { expose M::P; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_port_definition_browser.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b9db660f69d8d40c098868f903561f9d95f4f21aab6c00339fb08b8e992b5431") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::P"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~P") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::BrowserView")))))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "M::P")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "M::P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::P"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::P"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g")))
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
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_port_definition_browser.md") (range (start 2 13) (end 2 49)) (probe (position 2 13))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::BrowserView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView")))))
    )
  )
  (query (document "memory://snapshot/diagram_port_definition_browser.md") (range (start 2 59) (end 2 63)) (probe (position 2 59))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_browser.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "M::P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_port_definition_browser.md") (qualified-name "M::P")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 9,
  "modelDigest": "blake3:18b42a0889b59a49a1d9f81f35b87d3191085d5870afd53ca59fb050f45cb58f",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_port_definition_browser.md",
      "sourceDomain": "workspace"
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
      "qualifiedName": "M::g"
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
    }
  ],
  "selectedView": {
    "reference": 1,
    "kind": "browser-view",
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
        "origin": 0,
        "provenance": "authored",
        "reference": 3,
        "source": 1,
        "target": 0
      }
    ],
    "exposedRoots": [
      1
    ],
    "kind": "browser-view",
    "metadata": {
      "roots": [
        1
      ],
      "rows": [
        {
          "key": "98:element/v152:memory://snapshot/diagram_port_definition_browser.md7:packagen1:M1:08:port-defn1:P1:0124:element/v152:memory://snapshot/diagram_port_definition_browser.md7:packagen1:M1:08:port-defn1:P1:019:conjugated-port-defa1:0",
          "members": "none",
          "membership": "owned",
          "node": 0
        },
        {
          "key": "98:element/v152:memory://snapshot/diagram_port_definition_browser.md7:packagen1:M1:08:port-defn1:P1:0",
          "members": "loaded",
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
        "metaclass": "ConjugatedPortDefinition",
        "name": "~P",
        "notationRole": "definition",
        "owner": 1,
        "reference": 2,
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
      }
    ],
    "relationships": [],
    "scene": {
      "kind": "browser"
    }
  }
}

~~~
