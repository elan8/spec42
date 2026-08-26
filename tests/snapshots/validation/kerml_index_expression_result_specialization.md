# META
~~~ini
description=KerML checkIndexExpressionResultSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.6:checkIndexExpressionResultSpecialization
libraries=standard
~~~
# SOURCE
~~~kerml
package Model {
  feature values;
  feature index;
  feature indexed = values#(index);
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.6:checkIndexExpressionResultSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_index_expression_result_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:4ab99b3e7f8634533d00c3e6bc663cd7131c04bfc10fe3ab78205dfd3bccc6f5") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::index"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::indexed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (operator-expression (kind index) (arguments (argument (ordinal 0) (expression (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (argument (ordinal 1) (expression (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (result (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "values")) (expressionOperand (reference "index")))))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::values"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "values")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::values")))))
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "index")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::index")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::values"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::index"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::index"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::indexed"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::indexed"))) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::values"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::index")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::indexed")))
      (supertype (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::indexed")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::values")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_index_expression_result_specialization.md") (range (start 3 20) (end 3 26)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "values")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::values")))))
    )
  )
  (query (document "memory://snapshot/kerml_index_expression_result_specialization.md") (range (start 3 28) (end 3 33)) (probe (position 3 28))
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "indexed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "index")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_result_specialization.md") (qualified-name "Model::index")))))
    )
  )
)
~~~
