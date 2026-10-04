# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureResultRedefinition reports its unlowered prerequisite, not satisfaction, while an expression is not lowered as its own element
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureResultRedefinition
coverage_role=secondary
type=file
~~~
# SOURCE
~~~kerml
package Results {
    function Computing { in p; return r; }
    feature x;
    // The argument `x` is a FeatureReferenceExpression with its own result, which lowering does
    // not mint as an element.
    feature computed = Computing(x);
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureResultRedefinition") (outcome unsupported) (prerequisite expression_elements)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:74fe033b489d0b3570a9be6e30a65831abfd999f58e3a99d9d26e2ca473a2ddb"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::p"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::r"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::computed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")) (invocationCallee (reference "Computing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::x")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Computing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::p"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::r"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::computed"))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::p"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::r"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing"))) (supplied 1) (required 0) (start 5 23) (end 5 35))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::p")))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::r")))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::computed")))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::r")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::r")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::computed")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing::p")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (feature-reference "x" (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::x"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (range (start 5 33) (end 5 34)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::x")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (range (start 5 23) (end 5 32)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (path (named (kind package) (name "Results")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Computing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_result_redefinition_unlowered.md") (qualified-name "Results::Computing")))))
    )
  )
)
~~~
