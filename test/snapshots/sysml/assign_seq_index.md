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
    (element (id (node (document "d0") (qualified-name "AssignTest"))) (kind "package") (name "AssignTest") (declared-name "AssignTest") (range (start (line 0) (character 0)) (end (line 0) (character 145))))
    (element (id (node (document "d0") (qualified-name "AssignTest::A"))) (kind "action def") (name "A") (declared-name "A") (range (start (line 1) (character 4)) (end (line 1) (character 122))) (parent (node (document "d0") (qualified-name "AssignTest"))))
    (element (id (node (document "d0") (qualified-name "AssignTest::A::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 2) (character 8)) (end (line 2) (character 28))) (parent (node (document "d0") (qualified-name "AssignTest::A"))))
    (element (id (node (document "d0") (qualified-name "AssignTest::A::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 3) (character 8)) (end (line 3) (character 35))) (parent (node (document "d0") (qualified-name "AssignTest::A"))))
    (element (id (node (document "d0") (qualified-name "AssignTest::A::_assign#assign2"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 4) (character 8)) (end (line 4) (character 32))) (parent (node (document "d0") (qualified-name "AssignTest::A"))))
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
