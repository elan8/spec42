# META
~~~ini
description=Generate typed workspace requirement usages as CSV
type=generate
libraries=standard
plugin=requirement_instances_csv
~~~
# SOURCE
~~~sysml
package VehicleRequirements {
    part def Vehicle {
        attribute mass : ScalarValues::Real;
    }

    requirement def SafeStop {
        doc /*Definition of safe stopping.*/
    }
    requirement def LowBatteryWarning {
        doc /*Definition of battery warning.*/
    }

    requirement safeStopUsage : SafeStop {
        doc /*Usage for this vehicle.*/
    }
    requirement lowBatteryUsage : LowBatteryWarning;
    requirement missingTyping;
    requirement unresolvedTyping : MissingRequirementDefinition;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/requirement_instances_csv.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 35) (end 17 63))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:d83884a2c832875438b5c0a347e9e6aed46b36c320e42b730340af8eb6bb7686") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text "Definition of battery warning."))))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text "Definition of safe stopping."))))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::lowBatteryUsage"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LowBatteryWarning")))))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::missingTyping"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::safeStopUsage"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text "Usage for this vehicle."))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SafeStop")))))
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::unresolvedTyping"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingRequirementDefinition")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::lowBatteryUsage"))) (kind featureTyping) (ordinal 0))
      (authored-target "LowBatteryWarning")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning")))))
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::safeStopUsage"))) (kind featureTyping) (ordinal 0))
      (authored-target "SafeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop")))))
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::unresolvedTyping"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingRequirementDefinition")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::lowBatteryUsage"))) (target (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::lowBatteryUsage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::safeStopUsage"))) (target (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::safeStopUsage"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning")))
      (subtype (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::lowBatteryUsage")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop")))
      (subtype (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::safeStopUsage")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::lowBatteryUsage")))
      (type (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::safeStopUsage")))
      (type (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_instances_csv.md") (range (start 2 25) (end 2 43)) (probe (position 2 25))
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/requirement_instances_csv.md") (range (start 15 34) (end 15 51)) (probe (position 15 34))
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::lowBatteryUsage"))) (kind featureTyping) (ordinal 0) (authored-target "LowBatteryWarning")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::LowBatteryWarning")))))
    )
  )
  (query (document "memory://snapshot/requirement_instances_csv.md") (range (start 12 32) (end 12 40)) (probe (position 12 32))
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::safeStopUsage"))) (kind featureTyping) (ordinal 0) (authored-target "SafeStop")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::SafeStop")))))
    )
  )
  (query (document "memory://snapshot/requirement_instances_csv.md") (range (start 17 35) (end 17 63)) (probe (position 17 35))
    (reference (id (source (node (document "memory://snapshot/requirement_instances_csv.md") (qualified-name "VehicleRequirements::unresolvedTyping"))) (kind featureTyping) (ordinal 0) (authored-target "MissingRequirementDefinition")
      (outcome (status unresolved)))
    )
  )
)
~~~
# GENERATED
## requirement_instances.csv
~~~csv
qualified_name,name,requirement_definition_qualified_name,typing_status,typing_provenance
VehicleRequirements::lowBatteryUsage,lowBatteryUsage,VehicleRequirements::LowBatteryWarning,recovered,authored
VehicleRequirements::missingTyping,missingTyping,,missing-recovery,
VehicleRequirements::safeStopUsage,safeStopUsage,VehicleRequirements::SafeStop,recovered,authored
VehicleRequirements::unresolvedTyping,unresolvedTyping,,unresolved-recovery,

~~~
