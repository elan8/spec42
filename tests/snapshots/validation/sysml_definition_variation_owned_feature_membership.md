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
blocked_by=parser-gap-78-variation-forms
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;

    // Conforming: the variation owns only variant memberships.
    abstract variation part def Good {
        variant part small : Base;
        variant part large : Base;
    }

    // Invalid: the variation owns a plain feature membership.
    abstract variation part def Bad {
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
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 4 4) (end 10 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 4 4) (end 10 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:116e8c8ab0ab6ae98f9fe2882831b1990ce8c19361193d40cdba560e9451dbc2") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_owned_feature_membership.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
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
