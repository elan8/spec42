# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureOwnedReferenceSubsetting allows a Feature at most one ownedSubsetting that is a ReferenceSubsetting
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureOwnedReferenceSubsetting
type=file
skip_validation=sysml_resolution does not lower KermlFeatureMember.references -- the parser types the clause, lower_kerml_feature_member reads only typing/subsets/redefines -- so no ReferenceSubsetting relationship is published; the AST also holds a single Option, so a second references clause is silently discarded
~~~
# SOURCE
~~~kerml
package References {
    classifier Thing;
    classifier Holder {
        feature first : Thing;
        feature second : Thing;

        // Conforming: a single references clause.
        feature one references first;

        // Invalid: two references clauses on one feature.
        feature two references first references second;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_multiple_reference_subsettings")
        (source "semantic")
        (range (start 10 8) (end 10 55))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5e46d3170659298e90d529886f135498c767b204c48cd5be9bf1d2b7d913020b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (range (start 3 24) (end 3 29)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
    )
  )
)
~~~
