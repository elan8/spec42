# META
~~~ini
description=A unit symbol two admitted units answer to identifies neither of them
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package Ambiguity {
	package Local {
		attribute <kg> kilogramme : ISQ::MassUnit;
	}
	part def Vehicle {
		attribute mass : ISQ::MassValue = 1750 [kg];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/ambiguous_unit_symbol.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "ambiguous_unit_symbol")
        (source "semantic")
        (range (start 5 42) (end 5 44))
        (related-information
          (related
            (uri "memory://snapshot/sysml.library/si.md")
            (range (start 22 4) (end 22 127))
          )
          (related
            (uri "memory://snapshot/ambiguous_unit_symbol.md")
            (range (start 2 2) (end 2 44))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3426ad3e994f3126b103aa9ac2d5d026343880ba3944adde16b9e86a4683efb6") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (short-name "kg")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::MassUnit")))))
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::MassValue")))))
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::MassUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassUnit")))))
    (reference (id (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::MassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (target (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1750))) (unit "kg")))
    (unit (declaration (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "kg") (start 5 42) (end 5 44) (outcome (status ambiguous) (candidate (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::kilogram"))) (candidate (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme")))))
    (measurement (declaration (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassUnit"))))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassUnit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassUnit")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassUnit")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/ambiguous_unit_symbol.md") (path (named (kind package) (name "Ambiguity")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/ambiguous_unit_symbol.md") (range (start 2 30) (end 2 43)) (probe (position 2 30))
    (reference (id (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Local::kilogramme"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::MassUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassUnit")))))
    )
  )
  (query (document "memory://snapshot/ambiguous_unit_symbol.md") (range (start 5 19) (end 5 33)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/ambiguous_unit_symbol.md") (qualified-name "Ambiguity::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::MassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::MassValue")))))
    )
  )
)
~~~
