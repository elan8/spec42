# META
~~~ini
description=Abstract paired with variation is not a BasicDefinitionPrefix or RefPrefix spelling and must recover rather than parse as two modifiers
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;
    abstract variation part def IllegalDef;
    part def Holder {
        abstract variation part illegalUsage : Base;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/abstract_variation_is_unrepresentable.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 2 4) (end 2 43))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 2 4) (end 2 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:a55c2528f1937842330874de4d09701badaec4bc6009cfa6663ce8417f6ad681"))
  (declarations
    (declaration (id (node (document "memory://snapshot/abstract_variation_is_unrepresentable.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/abstract_variation_is_unrepresentable.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/abstract_variation_is_unrepresentable.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
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
