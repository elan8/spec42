# META
~~~ini
description=Fuzzer crash: unclosed short name with prefix metadata `#su<f` causes idempotence violation
type=file
~~~
# SOURCE
~~~sysml
package ion {
  class A {
    in f;
  }

  class A { in #su<f;
  }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 5 12) (end 6 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3243a503d57742c41e2e3f1a7c20618171f686f48bbd2205667150d514c83d00") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (qualified-name "ion"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A") (occurrence 1))))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (qualified-name "ion::A::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (qualified-name "ion::A::f"))) (target (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (qualified-name "ion::A::f")))
      (featured-by (node (document "memory://snapshot/fuzz_unclosed_short_name_metadata.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A")))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
