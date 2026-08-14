# META
~~~ini
description=Workspace typing resolved against the admitted standard library
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package Vehicles {
	part def Vehicle {
		attribute mass : ScalarValues::Real;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/standard_library_admission.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:bff19b16d920dfd8018b13865f28001c24bfa345a814844dd89ebaf4aa1ff1a1") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/standard_library_admission.md") (range (start 2 19) (end 2 37)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
  )
)
~~~
