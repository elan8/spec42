# META
~~~ini
description=SysML 8.3.6.5 validateVariantMembershipOwningNamespace requires the membershipOwningNamespace of a VariantMembership to be a variation-point Definition or Usage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.5 validateVariantMembershipOwningNamespace
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.5:validateVariantMembershipOwningNamespace
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;

    // Conforming: the variant membership is owned by a variation definition.
    variation part def Good {
        variant part small : Base;
    }

    // Invalid: every definition body admits `variant` members (DefinitionBodyItem), but the
    // owning namespace of a VariantMembership must be a variation.
    part def Plain {
        variant part small : Base;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_variant_membership_owning_namespace.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variant_outside_variation")
        (source "semantic")
        (range (start 11 16) (end 11 34))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_variant_membership_owning_namespace.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variant_outside_variation")
        (source "semantic")
        (range (start 11 16) (end 11 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:975f99430c4f56153ec588c21cbb64b0423486e667235cbc2a0116ca13027e5e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation) (implied-modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain::small"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain::small"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain::small"))) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain::small"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")))
      (subtype (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain::small")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small")))
      (type (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain::small")))
      (type (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (range (start 5 29) (end 5 33)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (range (start 11 29) (end 11 33)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Plain::small"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")))))
    )
  )
)
~~~
