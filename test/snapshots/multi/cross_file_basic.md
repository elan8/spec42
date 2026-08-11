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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "daf9f4108f74be994f93d4db20c6ca8595e5a701391f3fdc4f1cf03479715bb5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 0) (character 0)) (end (line 0) (character 97))))
    (element (id (node (document "d0") (qualified-name "Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 1) (character 4)) (end (line 1) (character 73))) (parent (node (document "d0") (qualified-name "Definitions"))))
    (element (id (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 2) (character 8)) (end (line 2) (character 44))) (parent (node (document "d0") (qualified-name "Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "ScalarValues::Real") (range (start (line 2) (character 25)) (end (line 2) (character 43)))))))
    (element (id (node (document "d1") (qualified-name "Usage"))) (kind "package") (name "Usage") (declared-name "Usage") (range (start (line 0) (character 0)) (end (line 0) (character 66))))
    (element (id (node (document "d1") (qualified-name "Usage::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 26))) (parent (node (document "d1") (qualified-name "Usage"))) (authored (membership (kind Import) (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 11)) (end (line 1) (character 22))))))
    (element (id (node (document "d1") (qualified-name "Usage::v"))) (kind "part") (name "v") (declared-name "v") (range (start (line 2) (character 4)) (end (line 2) (character 21))) (parent (node (document "d1") (qualified-name "Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 2) (character 13)) (end (line 2) (character 20)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Real") (range (start (line 2) (character 25)) (end (line 2) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d1") (qualified-name "Usage::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 1) (character 11)) (end (line 1) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Definitions")))))
    (reference (id (source (node (document "d1") (qualified-name "Usage::v"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 2) (character 13)) (end (line 2) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Definitions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d1") (qualified-name "Usage::v"))) (target (node (document "d0") (qualified-name "Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d1") (qualified-name "Usage::v"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
