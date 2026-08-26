# META
~~~ini
description=KerML 8.3.4.7.8 validateReturnParameterMembershipOwningType requires the owningType of a ReturnParameterMembership to be a Function or an Expression
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.8 validateReturnParameterMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.7.8:validateReturnParameterMembershipOwningType
blocked_by=semantic-return-parameter-membership-invalid-owner
type=file
~~~
# SOURCE
~~~kerml
package Returns {
    classifier Thing;

    // Conforming: the return parameter membership is owned by a function.
    function Computing {
        return feature result : Thing;
    }

    struct Object {
        // Invalid: a structure is neither a Function nor an Expression.
        return feature result : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "return_parameter_membership_invalid_owner")
        (source "semantic")
        (range (start 5 8) (end 5 38))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e5f2fdba6c1d57a562e7f98a97c4450cc5e00c7c9721709b326cfb19aa6c0e60") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result"))) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result"))) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result"))) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result"))) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result")))
      (featured-by (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing")))
      (type (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result")))
      (featured-by (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object")))
      (type (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")))
      (subtype (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (range (start 5 32) (end 5 37)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Computing::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (range (start 10 32) (end 10 37)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Object::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_return_parameter_membership_owning_type.md") (qualified-name "Returns::Thing")))))
    )
  )
)
~~~
