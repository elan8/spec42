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
# FORMAT
~~~sysml
package InvalidMult {
    classifier Bad [3..1];
    classifier AlsoBad [*..5];
    classifier Valid [1..3];
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "77c81d1c2fac3b9db063ddee3325842b91dd55f6a85c6c272194e7b5c1c59d8c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "InvalidMult"))) (kind "package") (name "InvalidMult") (declared-name "InvalidMult") (range (start (line 0) (character 0)) (end (line 0) (character 110))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::AlsoBad"))) (kind "classifier decl") (name "AlsoBad") (declared-name "AlsoBad") (range (start (line 2) (character 4)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "InvalidMult"))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::Bad"))) (kind "classifier decl") (name "Bad") (declared-name "Bad") (range (start (line 1) (character 4)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "InvalidMult"))))
    (element (id (node (document "d0") (qualified-name "InvalidMult::Valid"))) (kind "classifier decl") (name "Valid") (declared-name "Valid") (range (start (line 3) (character 4)) (end (line 3) (character 28))) (parent (node (document "d0") (qualified-name "InvalidMult"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
