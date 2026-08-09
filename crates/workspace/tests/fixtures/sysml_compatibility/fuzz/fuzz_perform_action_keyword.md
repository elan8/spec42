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
# EXPECTED
~~~
semantic.unresolved_name 'DoStuff'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DoStuff'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,OpenCurly,
KwFor,Ident,KwIn,Ident,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,
KwFor,Ident,KwIn,Ident,OpenCurly,CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (action_def 'A'
      (for_loop_node))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "P::A"))) (name "A") (declared-name "A")
          (contains
            (element (kind "for loop") (id (node (document "d0") (qualified-name "P::A::for_x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "P::A"))))
              (contains
                (element (kind "perform") (id (node (document "d0") (qualified-name "P::A::for_x::doStuff"))) (name "doStuff") (declared-name "doStuff") (effective (featuring-type (node (document "d0") (qualified-name "P::A")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz/fuzz_perform_action_keyword.md"
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
