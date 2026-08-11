# META
~~~ini
description=SysML part definition
type=file
~~~
# SOURCE
~~~sysml
part def Vehicle { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml_part_def.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
part def Vehicle { }

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d98bfc462bf660195d909f7d985d9b83b8814a3fd7b2284b376f80debb99a879") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 0) (character 0)) (end (line 0) (character 20))))
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
