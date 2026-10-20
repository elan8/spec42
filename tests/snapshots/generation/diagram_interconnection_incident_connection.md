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
  "schemaVersion": 6,
  "modelDigest": "blake3:f74c32f4bc89e383c50e4199d769627ee295b270ae0629a88db866ee3cf23470",
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
      "qualifiedName": "Model::selected"
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
      "source": 4,
      "ownerQualifiedName": "Model::Assembly",
      "metaclass": "ConnectionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "memberAccessOperand",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "typing",
      "ordinal": 0
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
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "connection",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 6
    }
  ],
  "selectedView": {
    "reference": 3,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 7
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "kind": "interconnection-view",
    "exposedRoots": [
      1,
      3
    ],
    "nodes": [
      {
        "reference": 7,
        "metaclass": "ConnectionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 1,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "left",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 4,
              "label": "Sensor"
            }
          ]
        },
        "owner": null,
        "source": 0,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              2
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
        "reference": 5,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "data",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Signal"
            }
          ]
        },
        "owner": 1,
        "source": 8,
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
        "name": "right",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 4,
              "label": "Sensor"
            }
          ]
        },
        "owner": null,
        "source": 2,
        "compartments": [
          {
            "kind": "ports",
            "provenance": "inherited",
            "members": [
              4
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
        "reference": 5,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "data",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Signal"
            }
          ]
        },
        "owner": 3,
        "source": 8,
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
        "reference": 11,
        "source": 1,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 4
        },
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 15,
        "source": 2,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 13,
        "source": 3,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 4
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 16,
        "source": 4,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 8,
        "source": 0,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 2
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 9,
        "source": 0,
        "kind": "memberAccessOperand",
        "target": {
          "status": "resolved",
          "node": 4
        },
        "provenance": "authored",
        "navigation": 6
      }
    ],
    "edges": [
      {
        "reference": 10,
        "source": 1,
        "target": 2,
        "origin": 2,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 12,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "noncomposite-membership",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 14,
        "source": 2,
        "target": 4,
        "origin": 0,
        "kind": "connector",
        "provenance": "authored",
        "navigation": 5
      }
    ],
    "metadata": {
      "parts": [
        1,
        3
      ],
      "ports": [
        2,
        4
      ],
      "connectors": [
        0
      ]
    },
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
