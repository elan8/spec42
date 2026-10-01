# META
~~~ini
description=KerML 8.3.4.6.4 validateParameterMembershipOwningType requires a ParameterMembership to be owned by a Behavior, a Step, or the result parameter of a ConstructorExpression
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.6.4 validateParameterMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/kerml/features.md
rule_id=kerml-1.0:8.3.4.6.4:validateParameterMembershipOwningType
type=file
~~~
# SOURCE
~~~kerml
// The violating side has no textual counterpart. KerML's only ParameterMembership productions are
// expression members (ArgumentMember, NamedArgumentMember, BodyParameterMember,
// TypeReferenceMember), owned by an Expression (a Step) or a constructor result; SysML adds them
// only in action-family productions owned by Steps. A directed `in feature` in a classifier body
// is an ordinary FeatureMembership: kerml/features.md pins Tanks::fuelInPort::fuelFlow as a plain
// directed kerml-feature member with no parameter role, as this fixture's SMG does Holder::input.
package Parameters {
    classifier Thing;

    // Conforming: the parameter membership is owned by a behavior.
    behavior Doing {
        in feature input : Thing;
    }

    classifier Holder {
        // Also conforming: a directed feature, not a ParameterMembership.
        in feature input : Thing;
    }
}
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cbbb7ac9e41af690dfbeb9c37f8f90d66b193871586f3fd3a12d752742e5e078"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder"))) (provenance implied))
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
  (query (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (range (start 11 27) (end 11 32)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Doing::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (range (start 16 27) (end 16 32)) (probe (position 16 27))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Holder::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_owning_type.md") (qualified-name "Parameters::Thing")))))
    )
  )
)
~~~
