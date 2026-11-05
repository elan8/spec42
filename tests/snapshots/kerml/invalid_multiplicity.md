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
      (diagnostic
        (severity warning)
        (code "invalid_multiplicity")
        (source "semantic")
        (range (start 1 19) (end 1 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:cc5c9e1665daf7691bc49b7f43bb745668e0c7a200ef1933ba6bb420971bcb83"))
  (declarations
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult::AlsoBad"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower unbounded) (upper 5))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (kind kerml-literal-infinity) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult::Bad"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 3) (upper 1))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (qualified-name "InvalidMult::Valid"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 3))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 5)))
    (evaluated (declaration (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 3)))
    (evaluated (declaration (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (state literal) (value (kind integer) (integer 3)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "AlsoBad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 5))))
  (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 3))))
  (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Bad")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/invalid_multiplicity.md") (path (named (kind package) (name "InvalidMult")) (named (kind kerml-classifier) (name "Valid")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (outcome resolved) (literal (value (kind integer) (integer 3))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
