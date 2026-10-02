# META
~~~ini
description=KerML 8.3.3.1.10 deriveTypeMultiplicity names the owned MultiplicityRange an authored [m..n] lowers to, and nothing for a Type without one
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Sized [1];
  type Unsized;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity")
    (source "Model::Sized")
    (outcome resolved))
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity")
    (source "Model::Unsized")
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_multiplicity.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:947dde742e4cc60adb3fa2fac0c97b48a3b918ca2b6e91fb915dceca8ae0efd2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model::Sized"))) (kind kerml-type) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model::Unsized"))) (kind kerml-type) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
