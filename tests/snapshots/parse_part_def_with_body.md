# META
~~~ini
description=Part def with nested part defs
type=file
~~~
# SOURCE
~~~sysml
part def Vehicle {
    part def Engine;
    part def Wheel;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parse_part_def_with_body.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:475a8acc0349832ea6d87241bcb909ecfc4da1c3d337392ef8c3426dc6eca23b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parse_part_def_with_body.md") (qualified-name "Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parse_part_def_with_body.md") (qualified-name "Vehicle::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parse_part_def_with_body.md") (qualified-name "Vehicle::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
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
