# META
~~~ini
description=Fuzzer crash: for loop with multiple trailing line comments in sequence causing idempotence violation
type=file
~~~
# SOURCE
~~~sysml
package P {
action def A {
    for
perform action doS : Dff {     for y // ndent g {
//'//ug {
// port for HTTPprin items { }
    }
    } }
}
~~~
# EXPECTED
~~~
parse.unexpected_token
semantic.unresolved_name 'Dff'
~~~
# PROBLEMS
~~~
parse.unexpected_token
semantic.unresolved_name 'Dff'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,OpenCurly,
KwFor,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,KwFor,Ident,LineComment,
LineComment,
LineComment,
CloseCurly,
CloseCurly,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (action_def 'A'
      (for_loop_node)
      (perform_action 'doS' : 'Dff'
        (for_loop_node))))
  (malformed))
~~~
# FORMAT
~~~sysml
package P {
    action def A {
        for in { }
        perform action doS : Dff {
            for y in { }
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
        (element (kind "action def") (id (node (document "d0") (qualified-name "P::A"))) (name "A") (declared-name "A"))
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
