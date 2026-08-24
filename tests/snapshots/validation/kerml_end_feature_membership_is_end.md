# META
~~~ini
description=KerML 8.3.3.3.3 validateEndFeatureMembershipIsEnd requires the ownedMemberFeature of an EndFeatureMembership to be an end Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.3 validateEndFeatureMembershipIsEnd
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.3:validateEndFeatureMembershipIsEnd
type=file
~~~
# SOURCE
~~~kerml
// Conforming: every EndFeatureMembership below is authored with the end keyword, which is what
// creates the membership and sets isEnd = true on the owned member feature at the same time.
//
// The violating side has no textual counterpart: KerML concrete syntax has no spelling that
// produces an EndFeatureMembership whose owned member feature is not an end feature, so the rule
// is observable only as the accepted side pinned here.
package Ends {
    classifier Thing;
    assoc Link {
        end feature source : Thing;
        end feature target : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_end_feature_membership_is_end.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_end_feature_membership_is_end.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bd31492d67bf80cc51b1c807edd46f0f7e8ee6d8207a40df69c6be1f1d2155e9") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source"))) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target"))) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source"))) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target"))) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link")))
      (type (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target")))
      (featured-by (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link")))
      (type (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (range (start 9 29) (end 9 34)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (range (start 10 29) (end 10 34)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_membership_is_end.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
