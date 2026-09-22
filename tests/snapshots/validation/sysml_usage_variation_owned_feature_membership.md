# META
~~~ini
description=SysML 8.3.6.4 validateUsageVariationOwnedFeatureMembership forbids a variation Usage from having any ownedFeatureMemberships
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.4 validateUsageVariationOwnedFeatureMembership
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.4:validateUsageVariationOwnedFeatureMembership
blocked_by=semantic-variation-owned-feature-membership
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;
    part def Holder {
        // Conforming: the variation usage owns only variant memberships.
        variation part good : Base {
            variant part small : Base;
            variant part large : Base;
        }

        // Invalid: the variation usage owns a plain feature membership.
        variation part bad : Base {
            variant part small : Base;
            part extra : Base;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variation_owns_feature_membership")
        (source "semantic")
        (range (start 12 12) (end 12 30))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d0d007e4e10c4e0844ea24eff93f166f4b8d9ffee72b2afb78173a2458dd59ea"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::small"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::large"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::small"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::small"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::large"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::small"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::small"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::small"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::large"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::large"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::small"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::small"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::small")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::large")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::small")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad")))
      (type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::small")))
      (type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::large")))
      (type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::small")))
      (type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (range (start 10 29) (end 10 33)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (range (start 12 25) (end 12 29)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::extra"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (range (start 11 33) (end 11 37)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::bad::small"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (range (start 4 30) (end 4 34)) (probe (position 4 30))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (range (start 6 33) (end 6 37)) (probe (position 6 33))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::large"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (range (start 5 33) (end 5 37)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder::good::small"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
)
~~~
