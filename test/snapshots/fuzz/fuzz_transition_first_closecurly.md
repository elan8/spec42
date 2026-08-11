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
  (document "fuzz_transition_first_closecurly.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 4 4) (end 4 23))
      )
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
        transition t first }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "f328845a657034629abb9dad491af60e1371c0ee0bc4b2e3fcb990aebb58db04") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
    (element (id (node (document "d0") (qualified-name "P::S"))) (kind "state def") (name "S") (declared-name "S") (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Owning)) (relationships (initial-state (reference "P::S::off")))))
    (element (id (node (document "d0") (qualified-name "P::S::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "P::S"))))
    (element (id (node (document "d0") (qualified-name "P::S::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "P::S"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::S"))) (kind initialStateSource) (ordinal 0)) (authored-target "P::S::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "P::S::off")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "P::S"))) (target (node (document "d0") (qualified-name "P::S::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P::S"))) (kind initialStateSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
