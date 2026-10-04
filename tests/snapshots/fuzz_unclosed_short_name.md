# META
~~~ini
description=Fuzzer crash: unclosed short name `<f` without `>` causes idempotence violation
type=file
~~~
# SOURCE
~~~sysml
package ion {
  class A {
    in<f;
  }

  class A { in #su f;
  }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_unclosed_short_name.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 2 4) (end 3 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 2 4) (end 3 2))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 5 2) (end 6 3))
        (related-information
          (related
            (uri "memory://snapshot/fuzz_unclosed_short_name.md")
            (range (start 1 2) (end 3 3))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3930c4b89a15b8382216b3ddbfb562c8f0dc22f978d8af0348670eed280d8607"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (qualified-name "ion"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A") (occurrence 1))))) (kind class-def) (membership (kind owning) (visibility default)))
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
# EXPRESSIONS
~~~sexpr
(expressions
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
