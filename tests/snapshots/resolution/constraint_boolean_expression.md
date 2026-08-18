# META
~~~ini
description=A constraint body that settles to a non-Boolean constant is reported; every other state is not
type=file
~~~
# SOURCE
~~~sysml
package Constraints {
	constraint def Ordered { 2 < 3 }
	constraint def Counted { 2 + 3 }
	constraint def Named { "approved" }
	constraint def Unresolved { missing < 3 }
	constraint def Unsupported { ~missing }
	part def Vehicle {
		attribute limit = 4;
		assert constraint withinLimit { limit }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/constraint_boolean_expression.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "non_boolean_expression")
        (source "semantic")
        (range (start 2 1) (end 2 33))
      )
      (diagnostic
        (severity warning)
        (code "non_boolean_expression")
        (source "semantic")
        (range (start 3 1) (end 3 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 29) (end 4 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 5 30) (end 5 38))
      )
      (diagnostic
        (severity warning)
        (code "non_boolean_expression")
        (source "semantic")
        (range (start 8 2) (end 8 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:49d90ed86cc45ef04d03e54393b5df1da3407c6a64032a57881d4da3cde92222") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Counted"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Named"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Ordered"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Unresolved"))) (kind constraint-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "missing")))))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Unsupported"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::limit"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::withinLimit"))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "limit")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Unresolved"))) (kind expressionOperand) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::withinLimit"))) (kind expressionOperand) (ordinal 0))
      (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::limit")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::withinLimit"))) (target (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::withinLimit"))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Counted"))) (state evaluated) (value (kind integer) (integer 5)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Named"))) (state literal) (value (kind string) (value "approved")))
    (evaluated (declaration (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Ordered"))) (state evaluated) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Unresolved"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Unsupported"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::limit"))) (state literal) (value (kind integer) (integer 4)))
    (evaluated (declaration (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::withinLimit"))) (state evaluated) (value (kind integer) (integer 4)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::limit")))
      (featured-by (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::withinLimit")))
      (featured-by (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/constraint_boolean_expression.md") (range (start 4 29) (end 4 36)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Unresolved"))) (kind expressionOperand) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/constraint_boolean_expression.md") (range (start 8 34) (end 8 39)) (probe (position 8 34))
    (reference (id (source (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::withinLimit"))) (kind expressionOperand) (ordinal 0) (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/constraint_boolean_expression.md") (qualified-name "Constraints::Vehicle::limit")))))
    )
  )
)
~~~
