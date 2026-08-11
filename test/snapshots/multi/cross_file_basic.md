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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Definitions"))) (name "Definitions") (declared-name "Definitions")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Definitions::Vehicle")))))
          )
        )
      )
    )
    (element (kind "package") (id (node (document "d1") (qualified-name "Usage"))) (name "Usage") (declared-name "Usage")
      (contains
        (element (kind "import") (id (node (document "d1") (qualified-name "Usage::*"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d1") (qualified-name "Usage::v"))) (name "v") (declared-name "v") (declared (properties (ordered false))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d1") (qualified-name "Usage::v"))) (to (node (document "d0") (qualified-name "Definitions::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Definitions::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Definitions::Vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d1") (qualified-name "Usage::v"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
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
    )
  )
  (document "Usage.sysml"
    (diagnostics
    )
  )
)
~~~
