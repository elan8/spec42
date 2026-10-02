# META
~~~ini
description=KerML 8.3.4.8.15 validateMetadataAccessExpressionReferencedElement requires a MetadataAccessExpression to have at least one ownedMember that is not a FeatureMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.15 validateMetadataAccessExpressionReferencedElement
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.15:validateMetadataAccessExpressionReferencedElement
blocked_by=lowering-metadata-expressions
type=file
~~~
# SOURCE
~~~kerml
// The violating side has no textual counterpart: MetadataAccessExpression is
// `referencedElement = [QualifiedName] '.' 'metadata'`, so every one owns its referenced element.
// Once the expression is lowered, the conforming side must report nothing.
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
        (range (start 7 23) (end 7 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:325337fac4603a1c70833618cc01df7ffe4c50704f3a0e7987307f2b91661dea"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0))))) (kind kerml-metadata-access-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta"))) (target (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta"))) (target (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta")))
      (featured-by (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (qualified-name "Expressions::Holder::meta")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_metadata_access_expression_referenced_element.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "meta")) (anonymous (kind kerml-metadata-access-expression) (ordinal 0))))) (outcome unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
