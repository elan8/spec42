# META
~~~ini
description=Fuzz: transition without 'then' keyword preserves middle tokens
type=file
~~~
# SOURCE
~~~sysml
package P {
    state def S {
        entry; then off;
        state off;
        transition t first off accept X state b;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_transition_no_then.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 4) (end 5 5))
      )
      (diagnostic
        (severity error)
        (code "recovered_state_body_element")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:0180b5949c33f6e3f29c98b26703a1155731639af5e631eb9c22d0c31a7d8e50") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_no_then.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S::off"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_transition_no_then.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S::off")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/fuzz_transition_no_then.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/fuzz_transition_no_then.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_transition_no_then.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S::off"))) (target (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_transition_no_then.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S")))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S::off")))
      (featured-by (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_transition_no_then.md") (range (start 2 20) (end 2 23)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/fuzz_transition_no_then.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_transition_no_then.md") (qualified-name "P::S::off")))))
    )
  )
)
~~~
