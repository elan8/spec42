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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 17) (end 2 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 37) (end 3 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 25) (end 4 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:058d3de74b320ebf59c3665087064fa28dc5af41d2a7abd88b7c720a8445f7ef") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (qualified-name "P::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0))))) (kind for-loop) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "seq")))))
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff"))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DoStuff")))))
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0))))) (kind for-loop) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "items")))))
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "y"))))) (kind for-loop-variable) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (kind for-loop-variable) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "seq")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff"))))) (kind featureTyping) (ordinal 0))
      (authored-target "DoStuff")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "items")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (qualified-name "P::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff"))))) (target (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "y"))))) (target (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (target (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (qualified-name "P::A")))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")))))
      (featured-by (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")))))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "y")))))
      (featured-by (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x")))))
      (featured-by (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_perform_action_keyword.md") (range (start 2 17) (end 2 20)) (probe (position 2 17))
    (reference (id (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "seq")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_perform_action_keyword.md") (range (start 3 37) (end 3 44)) (probe (position 3 37))
    (reference (id (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff"))))) (kind featureTyping) (ordinal 0) (authored-target "DoStuff")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_perform_action_keyword.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/fuzz_perform_action_keyword.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind for-loop) (ordinal 0)) (named (kind perform-action) (name "doStuff")) (anonymous (kind for-loop) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "items")
      (outcome (status unresolved)))
    )
  )
)
~~~
