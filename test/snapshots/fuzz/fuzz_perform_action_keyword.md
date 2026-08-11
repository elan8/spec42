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
# EXPECTED
~~~
semantic.unresolved_name 'DoStuff'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DoStuff'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "452114c6f16881f6044e71538c9407c493c370f920ab6154a635e5f4a1c58a26") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 167))))
    (element (id (node (document "d0") (qualified-name "P::A"))) (kind "action def") (name "A") (declared-name "A") (range (start (line 1) (character 4)) (end (line 1) (character 153))) (parent (node (document "d0") (qualified-name "P"))))
    (element (id (node (document "d0") (qualified-name "P::A::for_x"))) (kind "for loop") (name "x") (declared-name "x") (range (start (line 2) (character 8)) (end (line 2) (character 128))) (parent (node (document "d0") (qualified-name "P::A"))))
    (element (id (node (document "d0") (qualified-name "P::A::for_x::doStuff"))) (kind "perform") (name "doStuff") (declared-name "doStuff") (range (start (line 3) (character 12)) (end (line 3) (character 95))) (parent (node (document "d0") (qualified-name "P::A::for_x"))) (authored (relationships (typing (reference "DoStuff") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::A::for_x::doStuff"))) (kind featureTyping) (ordinal 0)) (authored-target "DoStuff") (range none) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
