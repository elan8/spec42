# META
~~~ini
description=Issue 297: retain action definition variant recovery
libraries=none
~~~
# SOURCE
~~~sysml
package BodyRecovery {
    action def A {
        part def Bad : ;
        item def Good;
        view def BadView : ;
        view def GoodView;
    }
    use case choices {
        variant use case bad : ;
        variant use case good;
    }
    state def S {
        variant state bad : ;
        variant state good;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_action_definition_variant_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "invalid_typing_operator")
        (source "parser")
        (range (start 2 8) (end 3 8))
      )
      (diagnostic
        (severity error)
        (code "recovered_use_case_body_element")
        (source "parser")
        (range (start 8 8) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "variant_outside_variation")
        (source "semantic")
        (range (start 9 16) (end 9 30))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 11 4) (end 14 5))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 12 8) (end 13 8))
      )
      (diagnostic
        (severity warning)
        (code "variant_outside_variation")
        (source "semantic")
        (range (start 13 16) (end 13 27))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_action_definition_variant_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "invalid_typing_operator")
        (source "parser")
        (range (start 2 8) (end 3 8))
      )
      (diagnostic
        (severity error)
        (code "recovered_use_case_body_element")
        (source "parser")
        (range (start 8 8) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "variant_outside_variation")
        (source "semantic")
        (range (start 9 16) (end 9 30))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 11 4) (end 14 5))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 12 8) (end 13 8))
      )
      (diagnostic
        (severity warning)
        (code "variant_outside_variation")
        (source "semantic")
        (range (start 13 16) (end 13 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9993dae76af3fb99521a6414537e2f91fc502428f3b8c50ce76cc47deeec579b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::A::BadView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::A::Good"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::A::GoodView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::S::good"))) (kind state) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::choices"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_definition_variant_recovery.md") (qualified-name "BodyRecovery::choices::good"))) (kind use-case) (membership (kind owning) (visibility default) (role variant)))
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
