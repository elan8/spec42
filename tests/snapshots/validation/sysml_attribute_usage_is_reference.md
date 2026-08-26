# META
~~~ini
description=SysML 8.3.7.3 validateAttributeUsageIsReference requires an AttributeUsage to be referential
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.7.3 validateAttributeUsageIsReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.7.3:validateAttributeUsageIsReference
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the attribute keyword produces an AttributeUsage and makes it referential at the
// same time.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling that
// produces a composite AttributeUsage, so the rule is observable only as the accepted side
// pinned here.
package Attributes {
    attribute def Reading;
    part def Holder {
        attribute measured : Reading;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_attribute_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_attribute_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c7224ceda181b91868479682911184d7207d315cd0f7ded6753cea42563d88b4") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured"))) (target (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured"))) (target (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured")))
      (featured-by (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder")))
      (type (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading")))
      (subtype (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (range (start 9 29) (end 9 36)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Holder::measured"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_is_reference.md") (qualified-name "Attributes::Reading")))))
    )
  )
)
~~~
