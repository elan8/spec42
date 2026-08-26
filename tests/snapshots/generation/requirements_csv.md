# META
~~~ini
description=Generate a CSV catalogue of workspace-authored system requirements
type=generate
libraries=standard
plugin=requirements_csv
~~~
# SOURCE
~~~sysml
package VehicleRequirements {
    part def Vehicle {
        attribute mass : ScalarValues::Real;
    }

    requirement def SafeStop {
        doc /*The vehicle shall stop safely after loss of propulsion.*/
    }

    requirement def LowBatteryWarning {
        doc /*The vehicle shall warn the operator before battery depletion.*/
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/requirements_csv.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:4ad1b8c2808c0284425350ca5b7b0e47a7586706a0f9fe5efe471632b3e9cc5e") (contract-version "constructor-expression-result-v8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text "The vehicle shall warn the operator before battery depletion."))))
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::SafeStop"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text "The vehicle shall stop safely after loss of propulsion."))))
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::SafeStop"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (target (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::SafeStop")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle")))
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
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirements_csv.md") (range (start 2 25) (end 2 43)) (probe (position 2 25))
    (reference (id (source (node (document "memory://snapshot/requirements_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
)
~~~
# GENERATED
## requirements.csv
~~~csv
qualified_name,name,documentation
VehicleRequirements::LowBatteryWarning,LowBatteryWarning,The vehicle shall warn the operator before battery depletion.
VehicleRequirements::SafeStop,SafeStop,The vehicle shall stop safely after loss of propulsion.

~~~
