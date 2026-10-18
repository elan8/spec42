# META
~~~ini
description=AST 284 extended and framed-concern values retain expression operands
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package UsageValues {
    metadata def Q;
    #Q def Extended;
    #Q decorated : Extended = missingExtended;
    concern def C;
    requirement def R {
        frame concern framed : C = missingConcern;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_usage_value_operands.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 30) (end 3 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 35) (end 6 49))
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind feature_typing) (source "UsageValues::decorated") (target "UsageValues::Extended") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "UsageValues::R::framed") (target "UsageValues::C") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_usage_value_operands.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 30) (end 3 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 35) (end 6 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2d19fdab8ba688c0815465ab77b93028e45208a37733971b365ddc4c73754ce2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C"))) (kind concern-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-definition) (name "Extended")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed"))) (kind frame) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C")))))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "missingConcern")))))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated"))) (kind extended-usage) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Extended")))))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "missingExtended")))))
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-definition) (name "Extended")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "missingConcern")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated"))) (kind featureTyping) (ordinal 0))
      (authored-target "Extended")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "missingExtended")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-definition) (name "Extended")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-definition) (name "Extended")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed"))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated"))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed"))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C")))
      (subtype (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended")))
      (subtype (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed")))
      (featured-by (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R")))
      (type (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated")))
      (type (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "missingConcern" (target unresolved)))
  (declaration (id (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "missingExtended" (target unresolved)))
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_usage_value_operands.md") (range (start 2 5) (end 2 6)) (probe (position 2 5))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-definition) (name "Extended")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_usage_value_operands.md") (range (start 6 31) (end 6 32)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::R::framed"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::C")))))
    )
  )
  (query (document "memory://snapshot/parser_284_usage_value_operands.md") (range (start 6 35) (end 6 49)) (probe (position 6 35))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind requirement-def) (name "R")) (named (kind frame) (name "framed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "missingConcern")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parser_284_usage_value_operands.md") (range (start 3 19) (end 3 27)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::decorated"))) (kind featureTyping) (ordinal 0) (authored-target "Extended")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Extended")))))
    )
  )
  (query (document "memory://snapshot/parser_284_usage_value_operands.md") (range (start 3 30) (end 3 45)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "missingExtended")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parser_284_usage_value_operands.md") (range (start 3 5) (end 3 6)) (probe (position 3 5))
    (reference (id (source (node (document "memory://snapshot/parser_284_usage_value_operands.md") (path (named (kind package) (name "UsageValues")) (named (kind extended-usage) (name "decorated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_usage_value_operands.md") (qualified-name "UsageValues::Q")))))
    )
  )
)
~~~
