# META
~~~ini
description=General view distinguishes authored and implied relationship provenance
type=generate
libraries=standard
plugin=repository:diagram
viewKind=general-view
viewDocument=diagram_relationship_provenance.md
viewQualifiedName=ProvenanceExample::selected
~~~
# SOURCE
~~~sysml
package ProvenanceExample {
    private import StandardViewDefinitions::*;
    part def Base { attribute mass : Real; }
    part def Child :> Base { attribute mass = 1200; }
    view selected : GeneralView { expose Child; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_relationship_provenance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 37) (end 2 41))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 3 29) (end 3 51))
        (related-information
          (related
            (uri "memory://snapshot/diagram_relationship_provenance.md")
            (range (start 2 20) (end 2 42))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:56ac1a41cff3d325151b7648ffcd7b27b65db0d0e4328768cadbb494f8c7cde4") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Child")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass"))) (state literal) (value (kind integer) (integer 1200)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass")))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))
      (subtype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child::mass")))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))
      (supertype (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 2 37) (end 2 41)) (probe (position 2 37))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 3 22) (end 3 26)) (probe (position 3 22))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Base")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 4 20) (end 4 31)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_relationship_provenance.md") (range (start 4 41) (end 4 46)) (probe (position 4 41))
    (reference (id (source (node (document "memory://snapshot/diagram_relationship_provenance.md") (path (named (kind package) (name "ProvenanceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_relationship_provenance.md") (qualified-name "ProvenanceExample::Child")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 2,
  "modelDigest": "blake3:f7608aaf74326f8d204133420bc63c009067e789a1a4627374ff9498681e4c8f",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_relationship_provenance.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        22,
        3,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        39,
        3,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        9,
        4,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Base"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Base::mass"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Child"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::Child::mass"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ProvenanceExample::selected"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "redefinition",
      "source": 3
    }
  ],
  "selectedView": {
    "reference": 4,
    "kind": "general-view",
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
        "navigation": 2,
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
        "metaclass": "PartDefinition",
        "name": "Child",
        "owner": null,
        "reference": 2,
        "source": 0
      },
      {
        "metaclass": "AttributeUsage",
        "name": "mass",
        "owner": 0,
        "reference": 3,
        "source": 2
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": 1,
        "provenance": "authored",
        "reference": 6,
        "source": 0,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 7,
        "source": 1,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      }
    ]
  }
}

~~~
