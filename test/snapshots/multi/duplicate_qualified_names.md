# META
~~~ini
description=Duplicate qualified names retain source-document identities
type=multi
~~~
# SOURCE
## alpha.sysml
~~~sysml
package P {
    part def Engine;
}
~~~
## beta.sysml
~~~sysml
package P {
    part def Engine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "alpha.sysml"
    (diagnostics
    )
  )
  (document "beta.sysml"
    (diagnostics
    )
  )
)
~~~
# FORMAT
## alpha.sysml
~~~sysml
package P {
    part def Engine;
}

~~~
## beta.sysml
~~~sysml
package P {
    part def Engine;
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "378dd26b42b8288df345a08ca74c0a9c71e69cd84be02185d886cb7fed584be4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 34))))
    (element (id (node (document "d0") (qualified-name "P::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 1) (character 4)) (end (line 1) (character 20))) (parent (node (document "d0") (qualified-name "P"))))
    (element (id (node (document "d1") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 34))))
    (element (id (node (document "d1") (qualified-name "P::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 1) (character 4)) (end (line 1) (character 20))) (parent (node (document "d1") (qualified-name "P"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
