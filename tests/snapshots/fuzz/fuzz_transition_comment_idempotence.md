# META
~~~ini
description=Fuzz: transition with line comment in absorbed tokens stops before comment
type=file
~~~
# SOURCE
~~~sysml
package j {
state def S {
    entry; then off;
    state off;
    transition t first accept X state package Timebehavior TakePicture          //ce [0..1];
                member step 'm' : ControlPerformances::MergePerformance [0..1] featured by TakePicture_snapshoure {
        public import 'merge';
}
                }

                // var step focus [0..1];               member step package RiskMetadataExEmple {
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_transition_comment_idempotence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_state_body_element")
        (source "parser")
        (range (start 4 4) (end 8 16))
      )
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 10 97) (end 10 97))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:0ceff0696a0ee462b1cb4998881875eb81b5204daf08aa8b9940af19de3e991c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S::off"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S::off")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S::off"))) (target (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S")))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S")))
    )
    (declaration (id (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S::off")))
      (featured-by (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (range (start 2 16) (end 2 19)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (path (named (kind package) (name "j")) (named (kind state-def) (name "S")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_transition_comment_idempotence.md") (qualified-name "j::S::off")))))
    )
  )
)
~~~
