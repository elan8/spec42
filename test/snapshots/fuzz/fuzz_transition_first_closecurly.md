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
# EXPECTED
~~~
semantic.duplicate_name 'off'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwState,KwDef,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,KwFirst,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (state_def 'S'
      (entry_action)
      (source_succession
        (default_ref_usage 'off'))
      (state_usage 'off')
      (transition_usage 't'))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P")
      (contains
        (element (kind "state def") (id (node (document "d0") (qualified-name "P::S"))) (name "S") (declared-name "S")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "P::S::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "P::S")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "P::S::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "P::S")))))
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "P::S"))) (to (node (document "d0") (qualified-name "P::S::off"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "P::S"))) (status missing-prerequisite) (target "States::StateAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "P::S::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "P::S::off"))) (status missing-prerequisite) (target "States::stateActions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz/fuzz_transition_first_closecurly.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 0) (end 1 74))
      )
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
