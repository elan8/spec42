# META
~~~ini
description=Fuzzer crash: send node with comment-only payload causing semicolon absorption
type=file
~~~
# SOURCE
~~~sysml
package P {
action def A {
    for
in send// nd port for HTT3prin  pq  for y  // nd port for HTT3prin items { }
  send pq   }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,OpenCurly,
KwFor,
KwIn,KwSend,LineComment,
KwSend,Ident,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (action_def 'A'
      (for_loop_node)
      (send_node)
      (send_node))))
~~~
# FORMAT
~~~sysml
package P {
    action def A {
        for
        in send// nd port for HTT3prin  pq  for y  // nd port for HTT3prin items { }
        send pq   }
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_send_comment_payload.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 2 4) (end 2 87))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 4 2) (end 4 12))
      )
    )
  )
)
~~~
