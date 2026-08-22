# META
~~~ini
description=A unit token in a publication with no measurement catalog is unresolvable, not unknown
type=file
~~~
# SOURCE
~~~sysml
package NoCatalog {
	part def Vehicle {
		attribute mass = 1750 [kg];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unit_catalog_unavailable.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a5e5fd48df91b7f545bd98fc2bbac11f6cb09c5a44ca904b5e6a749b567cf854") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass"))) (target (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass"))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1750))) (unit "kg")))
    (unit (declaration (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass"))) (ordinal 0) (authored "kg") (start 2 25) (end 2 27) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
