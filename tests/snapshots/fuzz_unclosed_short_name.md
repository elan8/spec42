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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 4) (end 2 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 7) (end 2 8))
      )
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:3930c4b89a15b8382216b3ddbfb562c8f0dc22f978d8af0348670eed280d8607") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (qualified-name "ion"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "in")) (expressionOperand (reference "f")))))
    (declaration (id (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A") (occurrence 1))))) (kind class-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (kind expressionOperand) (ordinal 0))
      (authored-target "in")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (kind expressionOperand) (ordinal 1))
      (authored-target "f")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (state unsupported))
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
  (query (document "memory://snapshot/fuzz_unclosed_short_name.md") (range (start 2 4) (end 2 6)) (probe (position 2 4))
    (reference (id (source (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (kind expressionOperand) (ordinal 0) (authored-target "in")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_unclosed_short_name.md") (range (start 2 7) (end 2 8)) (probe (position 2 7))
    (reference (id (source (node (document "memory://snapshot/fuzz_unclosed_short_name.md") (path (named (kind package) (name "ion")) (named (kind class-def) (name "A"))))) (kind expressionOperand) (ordinal 1) (authored-target "f")
      (outcome (status unresolved)))
    )
  )
)
~~~
