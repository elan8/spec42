# META
~~~ini
description=KerML 8.3.3.2.2 validateClassifierMultiplicityDomain requires the multiplicity of a Classifier to have no featuringTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.2.2 validateClassifierMultiplicityDomain
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.2.2:validateClassifierMultiplicityDomain
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the multiplicity of a classifier is declared on the classifier itself, so its
// domain is implicitly Base::Anything and it has no featuringTypes.
//
// The violating side has no textual counterpart: KerML concrete syntax attaches a classifier's
// multiplicity clause to the classifier declaration, and offers no spelling that gives that
// multiplicity a featuringType. The rule is observable only as the accepted side pinned here.
package Multiplicities {
    classifier Bounded[0..3];
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_classifier_multiplicity_domain.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_classifier_multiplicity_domain.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:51172a2516336843301f1ea9910ee3cb0e7d5271c0b394531e6db6ef2464a83a") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_classifier_multiplicity_domain.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_classifier_multiplicity_domain.md") (qualified-name "Multiplicities::Bounded"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 3))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
