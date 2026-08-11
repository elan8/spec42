# META
~~~ini
description=Fuzzer crash: unclosed short name `<f` without `>` causes idempotence violation
type=file
~~~
# SOURCE
~~~sysml
package ion {
  class A {
    in<f;
  }

  class A { in #su f;
  }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_unclosed_short_name.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ion {
  class A {
    in<f;
  }

  class A { in #su f;
  }
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fd3523005ae7cbf8914d9ae32e42b2eb2f315dc383ca9a8d32ecf20dd101b652") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ion"))) (kind "package") (name "ion") (declared-name "ion") (range (start (line 0) (character 0)) (end (line 0) (character 68))))
    (element (id (node (document "d0") (qualified-name "ion::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 2)) (end (line 1) (character 25))) (parent (node (document "d0") (qualified-name "ion"))))
    (element (id (node (document "d0") (qualified-name "ion::A#classifier_decl"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 5) (character 2)) (end (line 5) (character 25))) (parent (node (document "d0") (qualified-name "ion"))))
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
