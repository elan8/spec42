# META
~~~ini
description=Fuzz: var feature in definition body should not emit spurious 'member' keyword
type=file
~~~
# SOURCE
~~~sysml
package P {
    requirement r {
        var x :>> y = 42;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_member_var.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 2 8) (end 2 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:07f1b1ccc00ba750b93f193fa314818a3f3fd2d6f659e4ca9919ea073068fdd2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_member_var.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_member_var.md") (qualified-name "P::r"))) (kind requirement) (membership (kind feature) (visibility default)))
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
