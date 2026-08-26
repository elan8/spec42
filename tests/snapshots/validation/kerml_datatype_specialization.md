# META
~~~ini
description=KerML 8.3.4.1.2 validateDataTypeSpecialization forbids a DataType from specializing a Class or an Association
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.1.2 validateDataTypeSpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.1.2:validateDataTypeSpecialization
blocked_by=semantic-specialization-kind-compatibility
type=file
~~~
# SOURCE
~~~kerml
package Kinds {
    datatype Value;
    class Happening;

    // Conforming: a datatype specializes another datatype.
    datatype Refined specializes Value;

    // Invalid: a datatype must not specialize a class.
    datatype Wrong specializes Happening;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_datatype_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 8 4) (end 8 41))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_datatype_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ba755eb5eda4c3e8de29398d595c15844e5002114987af90acf3307f07fa0e68") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Happening"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Refined"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Value")))))
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Value"))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Wrong"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Happening")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Value")))))
    (reference (id (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0))
      (authored-target "Happening")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Happening")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Refined"))) (target (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Wrong"))) (target (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Happening"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Happening")))
      (subtype (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Wrong")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Refined")))
      (supertype (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Value")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Value")))
      (subtype (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Refined")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Wrong")))
      (supertype (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Happening")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_datatype_specialization.md") (range (start 5 33) (end 5 38)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Value")))))
    )
  )
  (query (document "memory://snapshot/kerml_datatype_specialization.md") (range (start 8 31) (end 8 40)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0) (authored-target "Happening")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_datatype_specialization.md") (qualified-name "Kinds::Happening")))))
    )
  )
)
~~~
