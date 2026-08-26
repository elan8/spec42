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
blocked_by=parser-gap-78-variation-forms
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;
    part def Holder {
        // Conforming: the variation usage owns only variant memberships.
        abstract variation part good : Base {
            variant part small : Base;
            variant part large : Base;
        }

        // Invalid: the variation usage owns a plain feature membership.
        abstract variation part bad : Base {
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
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 4 8) (end 10 8))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 4 8) (end 10 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:2459566b853712fa8a8ed60f26671913db33a4f4129922067b8f0c6b5d035778") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_owned_feature_membership.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
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
