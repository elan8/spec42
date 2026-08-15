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
  (document "memory://snapshot/invalid_multiplicity.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:84cfa0a389a471ec90c2ee39db33f0677f52c92146ede37f595699a772b912d8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult::AlsoBad"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower unbounded) (upper 5))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult::Bad"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 3) (upper 1))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult::Valid"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 3))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
