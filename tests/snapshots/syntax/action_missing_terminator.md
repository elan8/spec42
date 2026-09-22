# META
~~~ini
description=An action definition without a semicolon or brace body is not ActionBody and must recover
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Bare
    action def Next;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/action_missing_terminator.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_body_or_semicolon")
        (source "parser")
        (range (start 1 4) (end 2 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:187c653f3da3520f96f6f8f42256d05ba5febdd42642c8609f1c0bfa26ee5fd4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/action_missing_terminator.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/action_missing_terminator.md") (qualified-name "Actions::Next"))) (kind action-def) (membership (kind owning) (visibility default)))
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
