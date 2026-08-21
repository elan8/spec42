# META
~~~ini
description=KerML 8.3.4.6.4 validateParameterMembershipOwningType requires a ParameterMembership to be owned by a Behavior, a Step, or the result parameter of a ConstructorExpression
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.6.4 validateParameterMembershipOwningType
type=file
skip_validation=no semantic rule checks the owningType of a parameter membership; the canonical code parameter_membership_invalid_owner does not exist yet
~~~
# SOURCE
~~~kerml
package Parameters {
    classifier Thing;

    // Conforming: the parameter membership is owned by a behavior.
    behavior Doing {
        in feature input : Thing;
    }

    classifier Holder {
        // Invalid: a classifier is not a Behavior, a Step, or a constructor result parameter.
        in feature input : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_parameter_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "parameter_membership_invalid_owner")
        (source "semantic")
        (range (start 5 8) (end 5 33))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_parameter_membership_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4b240476e0f809eea67c3523455450568f731efd744377e8d501060ac40afade") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input")))
      (featured-by (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing")))
      (type (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input")))
      (featured-by (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder")))
      (type (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")))
      (subtype (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (range (start 5 27) (end 5 32)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (range (start 10 27) (end 10 32)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")))))
    )
  )
)
~~~
