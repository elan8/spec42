# META
~~~ini
description=KerML 8.3.4.7.3 checkExpressionResultBindingConnector over a SysML calculation usage: its body expression is a result expression bound to the usage's declared result
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.3:checkExpressionResultBindingConnector
type=file
~~~
# SOURCE
~~~sysml
package Constraints {
    attribute def Size;
    part def Box {
        attribute size : Size;
        calc doubled {
            return result : Size;
            size * 2
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.7.3:checkExpressionResultBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2f94c59c7307ffd0e22fcbbc6f51689da300ed3b6ef6260df8b18153d4cd817e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default) (role result-expression)) (relationships (expressionOperand (reference "size")))))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Size")))))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Size")))))
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "size")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size")))))
    (reference (id (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Size")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")))))
    (reference (id (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size"))) (kind featureTyping) (ordinal 0))
      (authored-target "Size")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result"))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size"))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled"))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result"))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size"))) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled")))
      (featured-by (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result")))
      (featured-by (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled")))
      (type (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size")))
      (featured-by (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box")))
      (type (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")))
      (subtype (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "*" (feature-reference "size" (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size")))) (literal (value (kind integer) (integer 2)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (range (start 6 12) (end 6 16)) (probe (position 6 12))
    (reference (id (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Box")) (named (kind calc) (name "doubled")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "size")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size")))))
    )
  )
  (query (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (range (start 5 28) (end 5 32)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::doubled::result"))) (kind featureTyping) (ordinal 0) (authored-target "Size")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")))))
    )
  )
  (query (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (range (start 3 25) (end 3 29)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Box::size"))) (kind featureTyping) (ordinal 0) (authored-target "Size")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_calculation_usage_result_binding_connector.md") (qualified-name "Constraints::Size")))))
    )
  )
)
~~~
