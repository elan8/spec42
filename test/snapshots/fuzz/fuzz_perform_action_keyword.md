# META
~~~ini
description=Fuzz: perform action preserves 'action' keyword for correct body parsing
type=file
~~~
# SOURCE
~~~sysml
package P {
    action def A {
        for x in seq {
            perform action doStuff : DoStuff {
                for y in items { }
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_perform_action_keyword.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 7 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:058d3de74b320ebf59c3665087064fa28dc5af41d2a7abd88b7c720a8445f7ef") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
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
