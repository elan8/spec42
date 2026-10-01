# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureParameterRedefinition reports its unlowered prerequisite, not satisfaction, while an operator expression parameter is not lowered
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureParameterRedefinition
coverage_role=secondary
type=file
~~~
# SOURCE
~~~kerml
package Parameters {
    function Computing { in p; in q; return r; }
    // The operator expression `1 + 2` owns two parameters the grammar defines and lowering does
    // not mint, so the positions they would pair are not facts.
    feature computed = Computing(1 + 2, 3);
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureParameterRedefinition") (outcome unsupported) (prerequisite grammar_parameters)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:09a1bd6ed1b334ec78fe2b387a0f862ea684a5b10c95570d2e8fdd30397d903c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::p"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::q"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::r"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::computed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "Computing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Computing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")))))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::p"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::q"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::r"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::computed"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::p"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::q"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::r"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing"))) (supplied 2) (required 0) (start 4 23) (end 4 42))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")))
      (subtype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::p")))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")))
      (subtype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::q")))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")))
      (subtype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::r")))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")))
      (subtype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::computed")))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::r")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::r")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::computed")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::p")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing::q")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (operator "+" (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 2)))) (literal (value (kind integer) (integer 3)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (range (start 4 23) (end 4 32)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (path (named (kind package) (name "Parameters")) (named (kind kerml-feature) (name "computed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Computing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition_unlowered.md") (qualified-name "Parameters::Computing")))))
    )
  )
)
~~~
