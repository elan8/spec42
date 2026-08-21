# META
~~~ini
description=SysML 8.3.6.5 validateVariantMembershipOwningNamespace requires the membershipOwningNamespace of a VariantMembership to be a variation-point Definition or Usage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.5 validateVariantMembershipOwningNamespace
type=file
skip_validation=a variant member parses but sysml_resolution reports it as unsupported_part_definition_member, so no VariantMembership reaches semantics
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;

    // Conforming: the variant membership is owned by a variation definition.
    abstract variation part def Good {
        variant part small : Base;
    }

    // Invalid: the variant membership is owned by a definition that is not a variation.
    part def Bad {
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
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 5 8) (end 5 34))
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
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 4 4) (end 9 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 10 8) (end 10 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:23e8da9f5b853900a4b4ee536707ce35558c2e727fa8ddb20867961e1739fbb4") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_variant_membership_owning_namespace.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
