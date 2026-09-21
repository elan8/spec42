# META
~~~ini
description=SysML 8.3.6.2 validateDefinitionVariationOwnedFeatureMembership forbids a variation Definition from having any ownedFeatureMemberships
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.2 validateDefinitionVariationOwnedFeatureMembership
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.2:validateDefinitionVariationOwnedFeatureMembership
blocked_by=semantic-variation-owned-feature-membership
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;

    // Conforming: the variation owns only variant memberships.
    // BasicDefinitionPrefix is `abstract` | `variation` (BNF 219), so variation is not paired
    // with abstract.
    variation part def Good {
        variant part small : Base;
        variant part large : Base;
    }

    // Invalid: the variation owns a plain feature membership.
    variation part def Bad {
        variant part small : Base;
        part extra : Base;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variation_owns_feature_membership")
        (source "semantic")
        (range (start 12 8) (end 12 26))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5bd2ffdad1d280d4a2462cff5283eedb8d6c299edb9db3e5a8266b652baa8c54"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::small"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::large"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::small"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::small"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::large"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra"))) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::small"))) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::small"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::large"))) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::large"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::small"))) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra"))) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra")))
      (featured-by (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad")))
      (type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::small")))
      (type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))
      (subtype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::small")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::large")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::small")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::large")))
      (type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::small")))
      (type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (range (start 14 21) (end 14 25)) (probe (position 14 21))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::extra"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (range (start 13 29) (end 13 33)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Bad::small"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (range (start 8 29) (end 8 33)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::large"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (range (start 7 29) (end 7 33)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base")))))
    )
  )
)
~~~
