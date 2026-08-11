# META
~~~ini
description=Assign node with sequence indexing operator #()
type=file
~~~
# SOURCE
~~~sysml
package AssignTest {
    action def A {
        assign x := seq#(i);
        assign 'var' := data#(idx);
        assign a.b := items#(0);
    }
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
KwAssign,Ident,ColonEq,Ident,Hash,OpenParen,Ident,CloseParen,Semicolon,
KwAssign,UnrestrictedName,ColonEq,Ident,Hash,OpenParen,Ident,CloseParen,Semicolon,
KwAssign,Ident,Dot,Ident,ColonEq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AssignTest'
    (action_def 'A'
      (assign_node)
      (assign_node)
      (assign_node))))
~~~
# FORMAT
~~~sysml
package AssignTest {
    action def A {
        assign x := seq#(i);
        assign 'var' := data#(idx);
        assign a.b := items#(0);
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AssignTest"))) (name "AssignTest") (declared-name "AssignTest")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "AssignTest::A"))) (name "A") (declared-name "A")
          (contains
            (element (kind "assign") (id (node (document "d0") (qualified-name "AssignTest::A::_assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "AssignTest::A")))))
            (element (kind "assign") (id (node (document "d0") (qualified-name "AssignTest::A::_assign#assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "AssignTest::A")))))
            (element (kind "assign") (id (node (document "d0") (qualified-name "AssignTest::A::_assign#assign2"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "AssignTest::A")))))
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
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignTest::A"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignTest::A::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignTest::A::_assign#assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AssignTest::A::_assign#assign2"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/assign_seq_index.md"
    (diagnostics
    )
  )
)
~~~
