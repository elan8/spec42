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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a5e5fd48df91b7f545bd98fc2bbac11f6cb09c5a44ca904b5e6a749b567cf854") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass"))) (target (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass"))) (target (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1750))) (unit "kg")))
    (unit (declaration (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "kg") (start 2 25) (end 2 27) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle")))
      (supertype (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/unit_catalog_unavailable.md") (path (named (kind package) (name "NoCatalog")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/unit_catalog_unavailable.md") (qualified-name "NoCatalog::Vehicle::mass")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
