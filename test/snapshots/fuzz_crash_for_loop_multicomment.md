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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_for_loop_multicomment.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 2 4) (end 2 109))
      )
      (diagnostic
        (severity error)
        (code "unexpected_closing_brace")
        (source "sysml")
        (range (start 8 0) (end 8 1))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b6fe1b3456fe1977a6494f3565b78acd31447e5a28448ac67dcedb8850b2fe5f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 139))))
    (element (id (node (document "d0") (qualified-name "P::A"))) (kind "action def") (name "A") (declared-name "A") (range (start (line 1) (character 0)) (end (line 1) (character 125))) (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
