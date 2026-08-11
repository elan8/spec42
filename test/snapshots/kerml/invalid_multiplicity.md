# META
~~~ini
description=KerML Invalid Multiplicity Ranges
type=file
~~~
# SOURCE
~~~kerml
package InvalidMult {
    classifier Bad [3..1];
    classifier AlsoBad [*..5];
    classifier Valid [1..3];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "invalid_multiplicity.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "77c81d1c2fac3b9db063ddee3325842b91dd55f6a85c6c272194e7b5c1c59d8c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "InvalidMult"))) (kind "package") (name "InvalidMult") (declared-name "InvalidMult"))
    (element (id (node (document "d0") (qualified-name "InvalidMult::AlsoBad"))) (kind "classifier decl") (name "AlsoBad") (declared-name "AlsoBad") (parent (node (document "d0") (qualified-name "InvalidMult"))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::Bad"))) (kind "classifier decl") (name "Bad") (declared-name "Bad") (parent (node (document "d0") (qualified-name "InvalidMult"))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::Valid"))) (kind "classifier decl") (name "Valid") (declared-name "Valid") (parent (node (document "d0") (qualified-name "InvalidMult"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
