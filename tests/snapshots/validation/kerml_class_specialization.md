# META
~~~ini
description=KerML 8.3.4.2.2 validateClassSpecialization forbids a Class from specializing a DataType, and from specializing an Association unless it is itself a kind of Association
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.2.2 validateClassSpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.2.2:validateClassSpecialization
blocked_by=semantic-specialization-kind-compatibility
type=file
~~~
# SOURCE
~~~kerml
package Kinds {
    datatype Value;
    class Happening;

    // Conforming: a class specializes another class.
    class Refined specializes Happening;

    // Invalid: a class must not specialize a datatype.
    class Wrong specializes Value;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_class_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 8 4) (end 8 34))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_class_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b3ecc3a78246632a765b2f5fa2b23792a19a7596f42cbfb695ce30001885f713") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Happening"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Refined"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Happening")))))
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Value"))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Wrong"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Value")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0))
      (authored-target "Happening")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Happening")))))
    (reference (id (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Value")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Refined"))) (target (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Happening"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Wrong"))) (target (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Happening")))
      (subtype (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Refined")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Refined")))
      (supertype (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Happening")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Value")))
      (subtype (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Wrong")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Wrong")))
      (supertype (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Value")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_class_specialization.md") (range (start 5 30) (end 5 39)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0) (authored-target "Happening")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Happening")))))
    )
  )
  (query (document "memory://snapshot/kerml_class_specialization.md") (range (start 8 28) (end 8 33)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_class_specialization.md") (qualified-name "Kinds::Value")))))
    )
  )
)
~~~
