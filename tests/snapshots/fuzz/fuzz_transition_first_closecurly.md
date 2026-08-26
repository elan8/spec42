# META
~~~ini
description=Fuzz: transition with 'first' ending at CloseCurly preserves name
type=file
~~~
# SOURCE
~~~sysml
package P {
state def S {
    entry; then off;
    state off;
    transition t first }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_transition_first_closecurly.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 0) (end 4 24))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 4 4) (end 4 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:7df16b8e3a470cadd6c4e2378e4ddc01ea22011846026fd5312dade18e131691"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S::off"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S::off")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S::off"))) (target (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S")))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S::off")))
      (featured-by (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_transition_first_closecurly.md") (range (start 2 16) (end 2 19)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_transition_first_closecurly.md") (qualified-name "P::S::off")))))
    )
  )
)
~~~
