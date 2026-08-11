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
  (document "Definitions.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 8) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 25) (end 2 43))
      )
    )
  )
  (document "Usage.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "daf9f4108f74be994f93d4db20c6ca8595e5a701391f3fdc4f1cf03479715bb5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions"))
    (element (id (node (document "d0") (qualified-name "Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Definitions"))))
    (element (id (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "ScalarValues::Real")))))
    (element (id (node (document "d1") (qualified-name "Usage"))) (kind "package") (name "Usage") (declared-name "Usage"))
    (element (id (node (document "d1") (qualified-name "Usage::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d1") (qualified-name "Usage"))) (authored (membership (kind Import) (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d1") (qualified-name "Usage::v"))) (kind "part") (name "v") (declared-name "v") (parent (node (document "d1") (qualified-name "Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d1") (qualified-name "Usage::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d1") (qualified-name "Usage::v"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Definitions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d1") (qualified-name "Usage::v"))) (target (node (document "d0") (qualified-name "Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d1") (qualified-name "Usage::v"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 25) (end 2 43)) (probe (position 2 25))
      (reference
        (source (document "d0") (qualified-name "Definitions::Vehicle::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "ScalarValues::Real")
        (range (start 2 25) (end 2 43))
        (outcome (status unresolved))
      )
    )
  )
  (document "d1"
    (query (range (start 2 13) (end 2 20)) (probe (position 2 13))
      (reference
        (source (document "d1") (qualified-name "Usage::v"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 2 13) (end 2 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Definitions::Vehicle") (range (start 1 4) (end 1 73)))
        )
      )
    )
    (query (range (start 1 11) (end 1 22)) (probe (position 1 11))
      (reference
        (source (document "d1") (qualified-name "Usage::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 1 11) (end 1 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Definitions") (range (start 0 0) (end 0 97)))
        )
      )
    )
  )
)
~~~
