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
  (document "fuzz_perform_action_keyword.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 12) (end 3 95))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c9db8128179b4b6976ae47dec53749b9a275973ddefe7959d91b8ef43cb80aa5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
    (element (id (node (document "d0") (qualified-name "P::A"))) (kind "action def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "P"))))
    (element (id (node (document "d0") (qualified-name "P::A::for_x"))) (kind "for loop") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "P::A"))))
    (element (id (node (document "d0") (qualified-name "P::A::for_x::doStuff"))) (kind "perform") (name "doStuff") (declared-name "doStuff") (parent (node (document "d0") (qualified-name "P::A::for_x"))) (authored (relationships (typing (reference "DoStuff")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::A::for_x::doStuff"))) (kind featureTyping) (ordinal 0)) (authored-target "DoStuff") (outcome (status unresolved)))
  )
  (relationships
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
