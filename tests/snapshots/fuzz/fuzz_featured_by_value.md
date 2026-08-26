# META
~~~ini
description=Fuzz: featured by must precede value assignment for idempotent reparse
type=file
~~~
# SOURCE
~~~sysml
package P {
    feature g featured by c = 42;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_featured_by_value.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 26) (end 1 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f019ca6c280207026f9327f0a96067d31cd9eeb75e2927843f541336a537d4b9") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_featured_by_value.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_featured_by_value.md") (qualified-name "P::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "c")))))
    (declaration (id (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_featured_by_value.md") (qualified-name "P::g"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "c")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/fuzz_featured_by_value.md") (qualified-name "P::g"))) (target (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 42)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_featured_by_value.md") (qualified-name "P::g")))
      (supertype (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_featured_by_value.md") (path (named (kind package) (name "P")) (named (kind kerml-feature) (name "g")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/fuzz_featured_by_value.md") (qualified-name "P::g")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_featured_by_value.md") (range (start 1 26) (end 1 27)) (probe (position 1 26))
    (reference (id (source (node (document "memory://snapshot/fuzz_featured_by_value.md") (qualified-name "P::g"))) (kind typeFeaturing) (ordinal 0) (authored-target "c")
      (outcome (status unresolved)))
    )
  )
)
~~~
