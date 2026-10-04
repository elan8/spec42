# META
~~~ini
description=Package containing part definitions
type=file
~~~
# SOURCE
~~~sysml
package Vehicles {
    part def Car;
    part def Truck;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parse_package_with_parts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f0ed2d7dd9d3194659d096a312fab72431f062b399c932fd53b5bf5f46894942"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parse_package_with_parts.md") (qualified-name "Vehicles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parse_package_with_parts.md") (qualified-name "Vehicles::Car"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parse_package_with_parts.md") (qualified-name "Vehicles::Truck"))) (kind part-def) (membership (kind owning) (visibility default)))
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
