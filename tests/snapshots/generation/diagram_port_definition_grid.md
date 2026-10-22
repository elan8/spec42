# META
~~~ini
description=Issue 284: GridView exposes a port definition and retains implicit conjugation without panicking
type=generate
libraries=standard
plugin=native:diagram
viewKind=grid-view
viewDocument=diagram_port_definition_grid.md
viewQualifiedName=M::g
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package M {
    port def P;
    view g : StandardViewDefinitions::GridView { expose M::P; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_port_definition_grid.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:58b91c5967075afcc353580f05fd0fcaba3db11c6f9139d87d51faa60ee9435c") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::P"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~P") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::GridView")))))
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "M::P")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::GridView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "M::P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::P"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::P"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_port_definition_grid.md") (range (start 2 13) (end 2 46)) (probe (position 2 13))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::g"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::GridView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")))))
    )
  )
  (query (document "memory://snapshot/diagram_port_definition_grid.md") (range (start 2 56) (end 2 60)) (probe (position 2 56))
    (reference (id (source (node (document "memory://snapshot/diagram_port_definition_grid.md") (path (named (kind package) (name "M")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "M::P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_port_definition_grid.md") (qualified-name "M::P")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:c8b06514870ac19b584a2174c04de0bab3b9fe4f6fcf93f3f4aa053b2c701dd3",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_port_definition_grid.md",
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
    "kind": "grid-view",
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
        "reference": 3,
        "source": 0,
        "target": 1
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "grid-view",
    "metadata": {
      "cells": [],
      "columns": [
        "specializes",
        "conjugation"
      ],
      "rows": [
        0,
        1
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
        "name": "~P",
        "notationRole": "definition",
        "owner": 0,
        "reference": 2,
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
      "kind": "grid"
    }
  }
}

~~~
