# META
~~~ini
description=A constraint / calc / value expression is published as a resolved tree: every operator kept, every feature reference paired with the specific feature it names, an unmodelled sub-shape kept as an unsupported node, and a wholly unmodelled shape reported as unsupported rather than dropped
type=file
~~~
# SOURCE
~~~sysml
package Expressions {
    part def Vehicle {
        attribute grossMass;
        attribute tareMass;
        attribute payloadMass;
        attribute axleCount;

        // Comparison over an arithmetic sub-expression: three authored feature references and a
        // literal, one `>=` and one `+`.
        constraint massBudget {
            grossMass >= tareMass + 100.0
        }

        // An accepted expression whose right operand is an invocation the slice does not model
        // yet: the `+` and both feature references are structured, the invocation is an
        // `unsupported` node that still lists what it ranges over.
        constraint withInvocation {
            grossMass == tareMass + round(payloadMass)
        }

        // A value expression: an arithmetic tree bound to a feature.
        attribute netMass = grossMass - tareMass;

        // A comparison whose left operand is a collection operator outside the slice: the `==`
        // and the literal are structured, `axleCount->size()` is an `unsupported` node.
        constraint balanced {
            axleCount->size() == 4
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/resolved_expression_tree.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 36) (end 17 41))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/resolved_expression_tree.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 36) (end 17 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f68ec606ecb86f160e5bc9ed23dca20c9023cd0a706e6089cdc299a202c68866"))
  (declarations
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::axleCount"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "axleCount")))))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "grossMass")) (expressionOperand (reference "tareMass")))))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::netMass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "grossMass")) (expressionOperand (reference "tareMass")))))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::payloadMass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "grossMass")) (expressionOperand (reference "tareMass")) (expressionOperand (reference "payloadMass")) (invocationCallee (reference "round")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (kind expressionOperand) (ordinal 0))
      (authored-target "axleCount")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::axleCount")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (kind expressionOperand) (ordinal 0))
      (authored-target "grossMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (kind expressionOperand) (ordinal 1))
      (authored-target "tareMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "grossMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "tareMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 0))
      (authored-target "grossMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 1))
      (authored-target "tareMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 2))
      (authored-target "payloadMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::payloadMass")))))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind invocationCallee) (ordinal 0))
      (authored-target "round")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::axleCount"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::payloadMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::axleCount"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::netMass"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::netMass"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::payloadMass"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::axleCount")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::netMass")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
      (supertype (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::netMass")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::payloadMass")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation")))
      (featured-by (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (outcome resolved) (operator "==" (unsupported (feature-reference "axleCount" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::axleCount"))))) (literal (value (kind integer) (integer 4)))))
  (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (outcome resolved) (operator ">=" (feature-reference "grossMass" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))) (operator "+" (feature-reference "tareMass" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))) (literal (value (kind real) (real 100))))))
  (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "-" (feature-reference "grossMass" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))) (feature-reference "tareMass" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass"))))))
  (declaration (id (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (outcome resolved) (operator "==" (feature-reference "grossMass" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))) (operator "+" (feature-reference "tareMass" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))) (unsupported (feature-reference "payloadMass" (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::payloadMass"))))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 26 12) (end 26 21)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::balanced"))) (kind expressionOperand) (ordinal 0) (authored-target "axleCount")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::axleCount")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 10 12) (end 10 21)) (probe (position 10 12))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (kind expressionOperand) (ordinal 0) (authored-target "grossMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 10 25) (end 10 33)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::massBudget"))) (kind expressionOperand) (ordinal 1) (authored-target "tareMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 21 28) (end 21 37)) (probe (position 21 28))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "grossMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 21 40) (end 21 48)) (probe (position 21 40))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (path (named (kind package) (name "Expressions")) (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "netMass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "tareMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 17 12) (end 17 21)) (probe (position 17 12))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 0) (authored-target "grossMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::grossMass")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 17 25) (end 17 33)) (probe (position 17 25))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 1) (authored-target "tareMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::tareMass")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 17 42) (end 17 53)) (probe (position 17 42))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind expressionOperand) (ordinal 2) (authored-target "payloadMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::payloadMass")))))
    )
  )
  (query (document "memory://snapshot/resolved_expression_tree.md") (range (start 17 36) (end 17 41)) (probe (position 17 36))
    (reference (id (source (node (document "memory://snapshot/resolved_expression_tree.md") (qualified-name "Expressions::Vehicle::withInvocation"))) (kind invocationCallee) (ordinal 0) (authored-target "round")
      (outcome (status unresolved)))
    )
  )
)
~~~
