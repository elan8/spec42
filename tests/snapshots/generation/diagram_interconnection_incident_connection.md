# META
~~~ini
description=Connections between separately exposed parts retain enclosing-assembly ownership; an unexposed inherited-port context is excluded (SysML 9.2.20.2.6)
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
        part hidden : Sensor;
        connect left.data to right.data;
        connect right.data to hidden.data;
    }
    part system : Assembly;
    view selected : InterconnectionView {
        expose system.left;
        expose system.right;
    }
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
        (range (start 3 4) (end 9 5))
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
        (range (start 11 4) (end 14 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:243d84d9ca650059960ffa1805a55097c5bdf9f4dafdb2fb5fb89878bebf9da8") (admitted (standard-library 1)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Vendor") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "left::data")) (memberAccessOperand (reference "right::data")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "right::data")) (memberAccessOperand (reference "hidden::data")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system::left")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system::right")))))
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
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "right::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "right::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "hidden::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")))))
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
      (authored-target "system::left")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0))
      (authored-target "system::right")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly")))
      (type (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (source direct))
      (supertype (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1)))))
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
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right"))) (terminal (resolved (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right")) (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) "right::data")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden"))) (terminal (resolved (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden")) (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data"))) "hidden::data")))
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
  (query (document "memory://snapshot/model.sysml") (range (start 8 16) (end 8 26)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "right::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 29) (end 7 39)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "right::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 30) (end 8 41)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Assembly")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "hidden::data")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor::data")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 22) (end 6 28)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::hidden"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "Vendor::Sensor")))))
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
  (query (document "memory://snapshot/model.sysml") (range (start 11 20) (end 11 39)) (probe (position 11 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::selected"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/vendor.sysml") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 15) (end 12 26)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "system::left")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::left")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 15) (end 13 27)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0) (authored-target "system::right")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Model::Assembly::right")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 18) (end 10 26)) (probe (position 10 18))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:f2a7a32f25a2ee6cf515e8ecece228c8ec678d6fdcefffd7719c310a9b91581b",
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
        11,
        9,
        11,
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
      "ordinal": 8,
      "relationshipKind": "memberAccessOperand",
      "source": 1
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
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "connection",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typing",
      "source": 6
    }
  ],
  "selectedView": {
    "reference": 4,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 7
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 2,
        "provenance": "implied",
        "reference": 12,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 4,
        "provenance": "implied",
        "reference": 15,
        "source": 3,
        "target": 4
      },
      {
        "kind": "connector",
        "navigation": 5,
        "origin": 0,
        "provenance": "authored",
        "reference": 18,
        "source": 2,
        "target": 4
      }
    ],
    "exposedRoots": [
      1,
      3
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [
        0
      ],
      "parts": [
        1,
        3
      ],
      "ports": [
        2,
        4
      ]
    },
    "nodes": [
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "reference": 8,
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
              2
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
        "owner": null,
        "reference": 2,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Sensor",
              "reference": 5
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
        "owner": 1,
        "reference": 6,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 7
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              4
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
        "owner": null,
        "reference": 3,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Sensor",
              "reference": 5
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
        "owner": 3,
        "reference": 6,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 7
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 14,
        "source": 1,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 13,
        "source": 1,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 21,
        "source": 2,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 19,
        "source": 2,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 17,
        "source": 3,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 16,
        "source": 3,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 22,
        "source": 4,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 20,
        "source": 4,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 5,
        "provenance": "authored",
        "reference": 9,
        "source": 0,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 6,
        "provenance": "authored",
        "reference": 10,
        "source": 0,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 11,
        "source": 0,
        "target": {
          "reference": 0,
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
