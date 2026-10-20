# META
~~~ini
description=Issue 297: malformed typing and state actions keep recovery diagnostics and retain later valid siblings
libraries=none
~~~
# SOURCE
~~~sysml
package Recovery284 {
    item def A;
    item def B;
    item broken : ;
    item cargo : A, B;
    action def Act;
    state def S {
        entry action broken : ;
        do action retained : Act;
        exit action finish : Act;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_typing_state_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 3 4) (end 3 19))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 3 4) (end 3 19))
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind feature_typing) (source "Recovery284::cargo") (target "Recovery284::A") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "Recovery284::cargo") (target "Recovery284::B") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "Recovery284::S::retained") (target "Recovery284::Act") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "Recovery284::S::finish") (target "Recovery284::Act") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_typing_state_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 3 4) (end 3 19))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 3 4) (end 3 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:caff4928aed00d5e90ea56b32d53c14c52a1c71212dc7e27113757af1a68ebd3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish"))) (kind exit-action-binding) (membership (kind feature) (visibility default) (role exit)) (authored (membership (kind feature) (visibility default) (role exit)) (relationships (featureTyping (reference "Act")))))
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained"))) (kind do-action-binding) (membership (kind feature) (visibility default) (role do)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (featureTyping (reference "Act")))))
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")) (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish"))) (kind featureTyping) (ordinal 0))
      (authored-target "Act")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained"))) (kind featureTyping) (ordinal 0))
      (authored-target "Act")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (kind featureTyping) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish"))) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained"))) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish"))) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained"))) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A")))
      (subtype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")))
      (subtype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B")))
      (subtype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish")))
      (featured-by (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S")))
      (type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained")))
      (featured-by (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S")))
      (type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo")))
      (type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A")) (provenance authored))
      (type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A")) (source direct))
      (effective-type (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A")) (scopes any))
      (supertype (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_typing_state_recovery.md") (range (start 9 29) (end 9 32)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::finish"))) (kind featureTyping) (ordinal 0) (authored-target "Act")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")))))
    )
  )
  (query (document "memory://snapshot/parser_284_typing_state_recovery.md") (range (start 8 29) (end 8 32)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::S::retained"))) (kind featureTyping) (ordinal 0) (authored-target "Act")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::Act")))))
    )
  )
  (query (document "memory://snapshot/parser_284_typing_state_recovery.md") (range (start 4 17) (end 4 18)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_typing_state_recovery.md") (range (start 4 20) (end 4 21)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::cargo"))) (kind featureTyping) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_typing_state_recovery.md") (qualified-name "Recovery284::B")))))
    )
  )
)
~~~
