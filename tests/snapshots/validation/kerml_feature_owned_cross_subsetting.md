# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureOwnedCrossSubsetting allows a Feature at most one ownedSubsetting that is a CrossSubsetting
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureOwnedCrossSubsetting
type=file
skip_validation=sysml_resolution does not lower KermlFeatureMember.crosses -- the parser types the clause, lower_kerml_feature_member reads only typing/subsets/redefines -- so no CrossSubsetting relationship is published; the AST also holds a single Option, so a second crosses clause is silently discarded
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing;
    assoc Link {
        end feature source : Thing;
        end feature target : Thing;

        // Conforming: a single crosses clause.
        feature one crosses source;

        // Invalid: two crosses clauses on one feature.
        feature two crosses source crosses target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_multiple_cross_subsettings")
        (source "semantic")
        (range (start 10 8) (end 10 50))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f03f31fa4546734ad2c2e5d09e0ea98827c105d80cea03fb3c1c6542e1008424") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 3 29) (end 3 34)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 4 29) (end 4 34)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
)
~~~
