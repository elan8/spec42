# META
~~~ini
description=Cross-file part def resolution with stdlib
type=multi
~~~
# SOURCE
## Definitions.sysml
~~~sysml
package Definitions {
    part def Vehicle {
        attribute mass : ScalarValues::Real;
    }
}
~~~
## Usage.sysml
~~~sysml
package Usage {
    import Definitions::*;
    part v : Vehicle;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/Definitions.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 25) (end 2 43))
      )
    )
  )
  (document "memory://snapshot/Usage.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7034a7a26b6fa6d3ae6735409e0ffb373f757bc8b3b9e3b20845f79655dd1f8b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Usage.sysml") (path (named (kind package) (name "Usage")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage::v"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/Usage.sysml") (path (named (kind package) (name "Usage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions")))))
    (reference (id (source (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage::v"))) (target (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage::v"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle")))
      (subtype (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage::v")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage::v")))
      (type (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/Definitions.sysml") (range (start 2 25) (end 2 43)) (probe (position 2 25))
    (reference (id (source (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/Usage.sysml") (range (start 1 11) (end 1 25)) (probe (position 1 11))
    (reference (id (source (node (document "memory://snapshot/Usage.sysml") (path (named (kind package) (name "Usage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions")))))
    )
  )
  (query (document "memory://snapshot/Usage.sysml") (range (start 2 13) (end 2 20)) (probe (position 2 13))
    (reference (id (source (node (document "memory://snapshot/Usage.sysml") (qualified-name "Usage::v"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/Definitions.sysml") (qualified-name "Definitions::Vehicle")))))
    )
  )
)
~~~
