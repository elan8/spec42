# META
~~~ini
description=KerML 8.3.3.1.9 checkMultiplicityTypeFeaturing requires a Multiplicity to inherit its owning Feature featuringTypes, or have none outside a Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.9:checkMultiplicityTypeFeaturing
type=file
~~~
# SOURCE
~~~kerml
package Multiplicities {
    classifier Vehicle {
        // The MultiplicityRange of `mass` is featured by `Vehicle`, like `mass` itself.
        feature mass [1];
    }
    // A Multiplicity outside a Feature, or of a Feature with no featuringType, has none.
    classifier Fleet [0..1];
    feature unfeatured [1];
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source (anonymous (owner "Multiplicities::Vehicle::mass") (kind MultiplicityRange) (ordinal 0)))
    (target "Multiplicities::Vehicle")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind type_featuring)
    (source (anonymous (owner "Multiplicities::Fleet") (kind MultiplicityRange) (ordinal 0)))
    (provenance implied)
    (outcome absent))
  (relationship
    (kind type_featuring)
    (source (anonymous (owner "Multiplicities::unfeatured") (kind MultiplicityRange) (ordinal 0)))
    (provenance implied)
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_multiplicity_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8cffdc868c8c3ba86509fa3b5eb1c66d3d0762a9f2d281254f03abacd6140fd1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Fleet"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::unfeatured"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Fleet")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (qualified-name "Multiplicities::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-classifier) (name "Vehicle")) (named (kind kerml-feature) (name "mass")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_multiplicity_type_featuring.md") (path (named (kind package) (name "Multiplicities")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
