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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "assign_seq_index.md"
    (diagnostics
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2c9232e8ce9fbcc6977f325dcc2469fb0eb768c0ac70dd12d88bf4bfa2036b0f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AssignTest"))) (kind "package") (name "AssignTest") (declared-name "AssignTest"))
    (element (id (node (document "d0") (qualified-name "AssignTest::A"))) (kind "action def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "AssignTest"))))
    (element (id (node (document "d0") (qualified-name "AssignTest::A::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "AssignTest::A"))))
    (element (id (node (document "d0") (qualified-name "AssignTest::A::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "AssignTest::A"))))
    (element (id (node (document "d0") (qualified-name "AssignTest::A::_assign#assign2"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "AssignTest::A"))))
  )
  (references
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
