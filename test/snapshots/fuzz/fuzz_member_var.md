# META
~~~ini
description=Fuzz: var feature in definition body should not emit spurious 'member' keyword
type=file
~~~
# SOURCE
~~~sysml
package P {
    requirement r {
        var x :>> y = 42;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_member_var.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    requirement r {
        var x :>> y = 42;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "561223b48f09d8c641a7fc81aabebbb0eb286f0e9a75fa37705ab0e529141c80") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 65))))
    (element (id (node (document "d0") (qualified-name "P::r"))) (kind "requirement") (name "r") (declared-name "r") (range (start (line 1) (character 4)) (end (line 1) (character 51))) (parent (node (document "d0") (qualified-name "P"))))
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
