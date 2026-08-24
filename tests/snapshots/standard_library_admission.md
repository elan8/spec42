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
	part def Car :> Vehicle {
		attribute :>> mass;
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:caecb804bb4b208a6a1b4c22abb097084eb87c6951dead58a85ce03fd63cae66") (contract-version "parser-owned-resolution-v2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle"))) (provenance implied))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car")))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source inherited) (from (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))))
      (supertype (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Car")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (subtype (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
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
  (query (document "memory://snapshot/standard_library_admission.md") (range (start 5 16) (end 5 20)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (path (named (kind package) (name "Vehicles")) (named (kind part-def) (name "Car")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass")))))
    )
  )
  (query (document "memory://snapshot/standard_library_admission.md") (range (start 2 19) (end 2 37)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/standard_library_admission.md") (qualified-name "Vehicles::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
)
~~~
