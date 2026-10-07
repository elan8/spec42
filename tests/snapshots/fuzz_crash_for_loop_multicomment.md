# META
~~~ini
description=Fuzzer crash: for loop with multiple trailing line comments in sequence causing idempotence violation
type=file
~~~
# SOURCE
~~~sysml
package P {
action def A {
    for
perform action doS : Dff {     for y // ndent g {
//'//ug {
// port for HTTPprin items { }
    }
    } }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_crash_for_loop_multicomment.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 2 4) (end 3 0))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 3 0) (end 7 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_closing_brace")
        (source "parser")
        (range (start 8 0) (end 8 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:5ac3bc1f2eccde1f0ac0a96375328a8951839d33059c4ea9c508c21b2ea97b1c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_crash_for_loop_multicomment.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_for_loop_multicomment.md") (qualified-name "P::A"))) (kind action-def) (membership (kind owning) (visibility default)))
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
