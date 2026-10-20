# META
~~~ini
description=An inherited library port is drawn only on the occurrence whose connector uses it (SysML 8.2.3.12/13)
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
        part spare : Sensor;
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
        (range (start 3 4) (end 8 5))
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
        (range (start 7 8) (end 7 40))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 7 8) (end 7 40))
      )
      (diagnostic
        (severity information)
        (code "missing_library_anchor")
        (source "semantic")
        (range (start 10 4) (end 10 58))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c781215f8de53c71e41d9e9a09c98de7ba91dd986b320ad34891bd4589853092") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Vendor") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "left::data")) (memberAccessOperand (reference "right::data")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::spare"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
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
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::spare"))) (kind featureTyping) (ordinal 0))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::spare"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::spare"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::spare"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::spare")))
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
  (query (document "memory://snapshot/model.sysml") (range (start 7 16) (end 7 25)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "left::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 29) (end 7 39)) (probe (position 7 29))
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
  (query (document "memory://snapshot/model.sysml") (range (start 6 21) (end 6 27)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::spare"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 20) (end 10 39)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 49) (end 10 55)) (probe (position 10 49))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 18) (end 9 26)) (probe (position 9 18))
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
  "schemaVersion": 6,
  "modelDigest": "blake3:d04ab0b1c70fab84c3920b70a3132066658b79b2593e588d19ecec65663c84f0",
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
        13,
        6,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        21,
        6,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        16,
        7,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        29,
        7,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        9,
        9,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        18,
        9,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        9,
        10,
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Model::Assembly"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Model::Assembly::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Model::Assembly::left"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Model::Assembly::right"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Model::Assembly::spare"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Model::selected"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Model::system"
    },
    {
      "kind": "qualified-name",
      "document": 1,
      "qualifiedName": "Vendor::Sensor"
    },
    {
      "kind": "qualified-name",
      "document": 1,
      "qualifiedName": "Vendor::Sensor::data"
    },
    {
      "kind": "qualified-name",
      "document": 1,
      "qualifiedName": "Vendor::Signal"
    },
    {
      "kind": "source-anchor",
      "source": 6,
      "ownerQualifiedName": "Model::Assembly",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "typing",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "typing",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "typing",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "connection",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "typing",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "typing",
      "ordinal": 7
    }
  ],
  "selectedView": {
    "reference": 5,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 11
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "kind": "interconnection-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 6,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "system",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Assembly"
            }
          ]
        },
        "owner": null,
        "source": 9,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "inherited",
            "members": [
              2,
              4,
              6
            ]
          },
          {
            "kind": "connections",
            "provenance": "inherited",
            "members": [
              1
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 10,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "left",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 7,
              "label": "Sensor"
            }
          ]
        },
        "owner": 0,
        "source": 0,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              3
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "data",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "Signal"
            }
          ]
        },
        "owner": 2,
        "source": 12,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 3,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "right",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 7,
              "label": "Sensor"
            }
          ]
        },
        "owner": 0,
        "source": 2,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              5
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "data",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 9,
              "label": "Signal"
            }
          ]
        },
        "owner": 4,
        "source": 12,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 4,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "spare",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 7,
              "label": "Sensor"
            }
          ]
        },
        "owner": 0,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 22,
        "source": 0,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 14,
        "source": 2,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 7
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 24,
        "source": 3,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 16,
        "source": 4,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 7
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 25,
        "source": 5,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 9
        },
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 17,
        "source": 6,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 7
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 11,
        "source": 1,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 3
        },
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 12,
        "source": 1,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 5
        },
        "provenance": "authored",
        "navigation": 8
      }
    ],
    "edges": [
      {
        "reference": 18,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 0
      },
      {
        "reference": 13,
        "source": 2,
        "target": 3,
        "origin": 3,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 12
      },
      {
        "reference": 19,
        "source": 0,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 2
      },
      {
        "reference": 15,
        "source": 4,
        "target": 5,
        "origin": 5,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 12
      },
      {
        "reference": 20,
        "source": 0,
        "target": 6,
        "origin": 6,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 4
      },
      {
        "reference": 21,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "implied",
        "navigation": 6
      },
      {
        "reference": 23,
        "source": 3,
        "target": 5,
        "origin": 1,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 7
      }
    ],
    "metadata": {
      "parts": [
        0,
        2,
        4,
        6
      ],
      "ports": [
        3,
        5
      ],
      "connectors": [
        1
      ]
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
