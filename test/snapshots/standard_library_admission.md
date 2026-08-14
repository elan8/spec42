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
	part def Car :> Vehicle;
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:8cd0ff72ecf52111076d9483d56a039f71edaa94f5305ec88dd6f96b856b735c") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car")))
      (supertype (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/standard_library_admission.md") (range (start 4 17) (end 4 24)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/standard_library_admission.md") (range (start 2 19) (end 2 37)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
)
~~~
