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
  feature sizedFeature [2];
  feature unsizedFeature;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity")
    (source "Model::sizedFeature")
    (outcome resolved))
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity")
    (source "Model::unsizedFeature")
    (outcome absent))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3c530369a8fd5744484cf4894805c58af1c6070ef7816c64d5a8e922712c30ba"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model::Sized"))) (kind kerml-type) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model::Unsized"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model::sizedFeature"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (qualified-name "Model::unsizedFeature"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 2)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-type) (name "Sized")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/kerml_type_multiplicity.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "sizedFeature")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 2))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
