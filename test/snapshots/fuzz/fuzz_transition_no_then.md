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
  (document "fuzz_transition_no_then.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    state def S {
        entry; then off;
        state off;
        transition t first off accept X state b;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d61b7f4656cd2bef75e811617796ff20fae173625997fd1731cf01fa70a2d20a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 130))))
    (element (id (node (document "d0") (qualified-name "P::S"))) (kind "state def") (name "S") (declared-name "S") (range (start (line 1) (character 4)) (end (line 1) (character 116))) (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Owning)) (relationships (initial-state (reference "P::S::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "P::S::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 2) (character 8)) (end (line 2) (character 14))) (parent (node (document "d0") (qualified-name "P::S"))))
    (element (id (node (document "d0") (qualified-name "P::S::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 3) (character 8)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "P::S"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::S"))) (kind initialStateSource) (ordinal 0)) (authored-target "P::S::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "P::S::off")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "P::S"))) (target (node (document "d0") (qualified-name "P::S::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P::S"))) (kind initialStateSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
