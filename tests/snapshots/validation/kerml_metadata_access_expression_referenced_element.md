# META
~~~ini
description=KerML 8.3.4.8.15 validateMetadataAccessExpressionReferencedElement requires a MetadataAccessExpression to have at least one ownedMember that is not a FeatureMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.15 validateMetadataAccessExpressionReferencedElement
type=file
skip_validation=a metadata access expression parses but sysml_resolution reports it as unsupported_calc_definition_member, so no MetadataAccessExpression reaches semantics; only the accepted side is authored here
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    classifier Holder {
        // Conforming: the metadata access expression names the element it reads.
        feature meta = Thing.metadata;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "metadata_access_missing_referenced_element")
        (source "semantic")
        (range (start 4 8) (end 4 38))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 4 23) (end 4 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:746c85cfbc34dea470230660129ec811abd13968cd93c4203394c588c8e29440") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta")))
      (featured-by (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
