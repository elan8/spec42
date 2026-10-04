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
  (document "memory://snapshot/sysml_part_def.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:28450cbff97f79df05bafe190cc9ce2026b3a89b2aafd6c5017ec9f06542a763"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_part_def.md") (qualified-name "Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
