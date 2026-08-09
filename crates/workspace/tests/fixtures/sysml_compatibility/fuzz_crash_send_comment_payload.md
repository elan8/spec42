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
        for in { }
        send;
        send pq;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (action_def 'A'
        (for_loop_action_usage)
        (send_action_usage)
        (send_action_usage)))))
~~~
