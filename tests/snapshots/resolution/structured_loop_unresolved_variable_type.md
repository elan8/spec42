# META
~~~ini
description=An authored for-loop variable type must resolve rather than being silently discarded
libraries=none
~~~
# SOURCE
~~~sysml
action def Process {
    for x : MissingType in (1, 2) {
        action visit { in value = x; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/structured_loop_unresolved_variable_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 12) (end 1 23))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/structured_loop_unresolved_variable_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 12) (end 1 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:aa47d09cfb60f271da85763ff4acad7b6255c6c464aed376e7a12a105d3d5764"))
  (declarations
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (qualified-name "Process"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0))))) (kind for-loop) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (value (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (kind for-loop-variable) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingType")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x")))))))
    (reference (id (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingType")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (qualified-name "Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit"))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value"))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (qualified-name "Process")))
    )
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")))))
      (featured-by (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")))))
      (featured-by (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")))))
    )
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")))))
    )
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x")))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x")))))
      (featured-by (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0))))) (outcome resolved) (unsupported (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 2)))))
  (declaration (id (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "x" (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x")))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (range (start 2 34) (end 2 35)) (probe (position 2 34))
    (reference (id (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "visit")) (named (kind parameter) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x")))))))
    )
  )
  (query (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (range (start 1 12) (end 1 23)) (probe (position 1 12))
    (reference (id (source (node (document "memory://snapshot/structured_loop_unresolved_variable_type.md") (path (named (kind action-def) (name "Process")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "x"))))) (kind featureTyping) (ordinal 0) (authored-target "MissingType")
      (outcome (status unresolved)))
    )
  )
)
~~~
