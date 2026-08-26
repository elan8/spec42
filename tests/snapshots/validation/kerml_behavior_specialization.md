# META
~~~ini
description=KerML 8.3.4.6.2 validateBehaviorSpecialization forbids a Behavior from specializing a Structure
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.6.2 validateBehaviorSpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.6.2:validateBehaviorSpecialization
blocked_by=semantic-specialization-kind-compatibility
type=file
~~~
# SOURCE
~~~kerml
package Kinds {
    struct Object;
    behavior Doing;

    // Conforming: a behavior specializes another behavior.
    behavior Refined specializes Doing;

    // Invalid: a behavior must not specialize a structure.
    behavior Wrong specializes Object;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_behavior_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 8 4) (end 8 38))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_behavior_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e133a6e80e121e7dc0531e70e6e139d0f90b981b137e20a4334293f50f5f62c2") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Doing"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Object"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Refined"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Doing")))))
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Wrong"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0))
      (authored-target "Doing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Doing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Object")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Refined"))) (target (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Doing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Wrong"))) (target (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Object"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Doing")))
      (subtype (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Refined")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Object")))
      (subtype (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Wrong")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Refined")))
      (supertype (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Doing")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Wrong")))
      (supertype (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Object")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_behavior_specialization.md") (range (start 5 33) (end 5 38)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0) (authored-target "Doing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Doing")))))
    )
  )
  (query (document "memory://snapshot/kerml_behavior_specialization.md") (range (start 8 31) (end 8 37)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_behavior_specialization.md") (qualified-name "Kinds::Object")))))
    )
  )
)
~~~
