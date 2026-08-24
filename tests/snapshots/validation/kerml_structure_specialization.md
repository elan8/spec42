# META
~~~ini
description=KerML 8.3.4.3.2 validateStructureSpecialization forbids a Structure from specializing a Behavior
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.3.2 validateStructureSpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.3.2:validateStructureSpecialization
blocked_by=semantic-specialization-kind-compatibility
type=file
~~~
# SOURCE
~~~kerml
package Kinds {
    struct Object;
    behavior Doing;

    // Conforming: a structure specializes another structure.
    struct Refined specializes Object;

    // Invalid: a structure must not specialize a behavior.
    struct Wrong specializes Doing;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_structure_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 8 4) (end 8 35))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_structure_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2508056fb255af3fad663b304fe635ced5be39d17ccc46b94e6756c7b064c865") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Doing"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Object"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Refined"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object")))))
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Wrong"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Doing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Object")))))
    (reference (id (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0))
      (authored-target "Doing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Doing")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Refined"))) (target (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Object"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Wrong"))) (target (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Doing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Doing")))
      (subtype (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Wrong")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Object")))
      (subtype (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Refined")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Refined")))
      (supertype (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Object")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Wrong")))
      (supertype (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Doing")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_structure_specialization.md") (range (start 5 31) (end 5 37)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Refined"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Object")))))
    )
  )
  (query (document "memory://snapshot/kerml_structure_specialization.md") (range (start 8 29) (end 8 34)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Wrong"))) (kind specialization) (ordinal 0) (authored-target "Doing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_structure_specialization.md") (qualified-name "Kinds::Doing")))))
    )
  )
)
~~~
