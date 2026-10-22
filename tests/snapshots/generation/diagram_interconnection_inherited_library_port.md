# META
~~~ini
description=An inherited library port remains a contextual connector endpoint in an interconnection view (SysML 8.2.3.12/13)
type=generate
libraries=none
standard_library_document=vendor.sysml
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=Model::selected
~~~
# SOURCE
## vendor.sysml
~~~sysml
package Vendor {
    port def Signal;
    part def Sensor { port data : Signal; }
}
standard library package StandardViewDefinitions {
    view def InterconnectionView;
}
~~~
## model.sysml
~~~sysml
package Model {
    private import StandardViewDefinitions::*;
    private import Vendor::*;
    part def Assembly {
        part left : Sensor;
        part right : Sensor;
        connect left.data to right.data;
    }
    part system : Assembly;
    view selected : InterconnectionView { expose system; }
}
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
        (range (start 3 4) (end 7 5))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 4 8) (end 4 27))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 6 8) (end 6 40))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 6 8) (end 6 40))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 9 4) (end 9 58))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:861b3113156051197223f0d2e9d64bad83e453daa9b2be7565924c6a145d351b") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Vendor") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "left::data")) (memberAccessOperand (reference "right::data")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Assembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Vendor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "left::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "right::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))
      (type (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (source direct))
      (supertype (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))
      (type (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (source direct))
      (supertype (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected")))
      (type (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source direct))
      (supertype (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")) (scopes any))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (terminal (resolved (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left")) (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) "left::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (terminal (resolved (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right")) (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) "right::data")))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 28)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Vendor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 16) (end 6 25)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "left::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 29) (end 6 39)) (probe (position 6 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "right::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 20) (end 4 26)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 21) (end 5 27)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 20) (end 9 39)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 49) (end 9 55)) (probe (position 9 49))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 18) (end 8 26)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (kind featureTyping) (ordinal 0) (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 7,
  "modelDigest": "blake3:101af4ed4e9ee7f5f919aa2138312085c6135e9fe131e6152970b26854020937",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/vendor.sysml",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
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
        4,
        20,
        4,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        21,
        5,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        8,
        6,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        16,
        6,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        29,
        6,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        9,
        8,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        18,
        8,
        26
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
    },
    {
      "document": 1,
      "range": [
        2,
        27,
        2,
        31
      ]
    },
    {
      "document": 1,
      "range": [
        2,
        34,
        2,
        40
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Model::Assembly"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Model::Assembly::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Model::Assembly::left"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Model::Assembly::right"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Model::selected"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Model::system"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Vendor::Sensor"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Vendor::Sensor::data"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Vendor::Signal"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Model::Assembly",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "memberAccessOperand",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "memberAccessOperand",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "noncomposite-membership",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "connection",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typing",
      "source": 7
    }
  ],
  "selectedView": {
    "reference": 4,
    "kind": "interconnection-view",
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
        "navigation": 0,
        "origin": 2,
        "provenance": "implied",
        "reference": 16,
        "source": 0,
        "target": 2
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 3,
        "provenance": "implied",
        "reference": 12,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 4,
        "provenance": "implied",
        "reference": 17,
        "source": 0,
        "target": 4
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 5,
        "provenance": "implied",
        "reference": 14,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 1,
        "provenance": "implied",
        "reference": 18,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 5,
        "origin": 1,
        "provenance": "authored",
        "reference": 20,
        "source": 3,
        "target": 5
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [
        1
      ],
      "parts": [
        0,
        2,
        4
      ],
      "ports": [
        3,
        5
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              2,
              4
            ],
            "provenance": "inherited"
          },
          {
            "kind": "connections",
            "members": [
              1
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "system",
        "notationRole": "usage",
        "owner": null,
        "reference": 5,
        "source": 7,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Assembly",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 9,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              3
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "left",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Sensor",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "name": "data",
        "notationRole": "usage",
        "owner": 2,
        "reference": 7,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 8
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              5
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "right",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Sensor",
              "reference": 6
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "name": "data",
        "notationRole": "usage",
        "owner": 4,
        "reference": 7,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 8
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 19,
        "source": 0,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 13,
        "source": 2,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 21,
        "source": 3,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 15,
        "source": 4,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 22,
        "source": 5,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 5,
        "provenance": "authored",
        "reference": 10,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 6,
        "provenance": "authored",
        "reference": 11,
        "source": 1,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
