# META
~~~ini
description=A constraint or calculation body expression is a result expression element that owns its evaluation: comparisons and arithmetic fold, a negative integer exponent promotes to Real, operands resolve from the element, and an expression over non-constant operands stays NonConstant
type=file
~~~
# SOURCE
~~~sysml
package Demo {
    attribute a;
    attribute b;
    attribute c;
    constraint def Compare { 1 < 2 }
    constraint def Budget { (a + b) < c }
    calc def Add { 2 + 3 }
    calc def Power { 2 ^ -1 }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/result_expression_evaluation.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:39f6b1d7f4c6da0cf5307936a003de25c6359adb5d07f5bc001ddd6298913e5b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Add"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Budget"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default) (role result-expression)) (relationships (expressionOperand (reference "a")) (expressionOperand (reference "b")) (expressionOperand (reference "c")))))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Compare"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Power"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::a"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::b"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::c"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::a")))))
    (reference (id (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::b")))))
    (reference (id (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::c")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Add"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Budget"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Compare"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Power"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0))))) (state evaluated) (value (kind integer) (integer 5)))
    (evaluated (declaration (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0))))) (state evaluated) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0))))) (state evaluated) (value (kind real) (real 0.5)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Add")))
    )
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Budget")))
    )
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Compare")))
    )
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::Power")))
    )
    (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Add")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "+" (literal (value (kind integer) (integer 2))) (literal (value (kind integer) (integer 3)))))
  (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "<" (operator "+" (feature-reference "a" (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::a")))) (feature-reference "b" (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::b"))))) (feature-reference "c" (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::c"))))))
  (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Compare")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "<" (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 2)))))
  (declaration (id (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind calc-def) (name "Power")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "^" (literal (value (kind integer) (integer 2))) (operator "-" (literal (value (kind integer) (integer 1))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/result_expression_evaluation.md") (range (start 5 29) (end 5 30)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::a")))))
    )
  )
  (query (document "memory://snapshot/result_expression_evaluation.md") (range (start 5 33) (end 5 34)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::b")))))
    )
  )
  (query (document "memory://snapshot/result_expression_evaluation.md") (range (start 5 38) (end 5 39)) (probe (position 5 38))
    (reference (id (source (node (document "memory://snapshot/result_expression_evaluation.md") (path (named (kind package) (name "Demo")) (named (kind constraint-def) (name "Budget")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/result_expression_evaluation.md") (qualified-name "Demo::c")))))
    )
  )
)
~~~
