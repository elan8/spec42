# META
~~~ini
description=SysML 8.3.6.5 validateVariantMembershipOwningNamespace requires the membershipOwningNamespace of a VariantMembership to be a variation-point Definition or Usage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.5 validateVariantMembershipOwningNamespace
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.6.5:validateVariantMembershipOwningNamespace
blocked_by=abstract-syntax-invalid-variant-owner
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
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_variant_membership_owning_namespace.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:02f1e99959e9b6b329c3850fb8078fa6ef1e648f4746fe9ea1d4e4e4ba6f8b89") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (target (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small"))) (kind featureTyping) (ordinal 0)))
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
    )
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Good::small")))
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
)
~~~
