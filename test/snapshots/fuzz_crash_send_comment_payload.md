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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "d53ebad1f320716a2531ac6fd8d40ee848774b5bfca9ee9291ca1996f6eb8f51") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 127))))
    (element (id (node (document "d0") (qualified-name "P::A"))) (kind "action def") (name "A") (declared-name "A") (range (start (line 1) (character 0)) (end (line 1) (character 113))) (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
