# META
~~~ini
description=KerML 8.3.4.6.4 validateParameterMembershipParameterDirection requires the ownedMemberParameter of a ParameterMembership to have the direction returned by parameterDirection()
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.6.4 validateParameterMembershipParameterDirection
type=file
~~~
# SOURCE
~~~kerml
// Conforming: each parameter below carries exactly the direction its ParameterMembership kind
// prescribes -- in, out and inout come from the direction keyword itself, and the return
// parameter's direction comes from the return keyword.
//
// The violating side has no textual counterpart: KerML concrete syntax derives both the
// ParameterMembership kind and the parameter's direction from the same keyword, so a source
// document cannot author a parameter whose direction disagrees with its membership.
package Parameters {
    classifier Thing;
    function Computing {
        in feature input : Thing;
        out feature output : Thing;
        inout feature both : Thing;
        return feature result : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d9ca3ac205dd0c088c18a9b081bd402e524a22025a36f230111c54bd0a693d76") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::both"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction inout)))))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::output"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction out)))))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::both"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
  )
  (relationships
    (relationship (kind typing) (direction inout) (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::both"))) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::both"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::input"))) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::output"))) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::output"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::result"))) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::result"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::both")))
      (featured-by (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing")))
      (type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::input")))
      (featured-by (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing")))
      (type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::output")))
      (featured-by (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing")))
      (type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::result")))
      (featured-by (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing")))
      (type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))
      (subtype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::both")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::output")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (range (start 12 29) (end 12 34)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::both"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (range (start 10 27) (end 10 32)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (range (start 11 29) (end 11 34)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::output"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (range (start 13 32) (end 13 37)) (probe (position 13 32))
    (reference (id (source (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Computing::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_parameter_membership_parameter_direction.md") (qualified-name "Parameters::Thing")))))
    )
  )
)
~~~
