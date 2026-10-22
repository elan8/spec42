# META
~~~ini
description=KerML Simple Tests: Expansion
type=file
~~~
# SOURCE
~~~kerml
package Expansion {
	private import ControlFunctions::select;
	feature x = x->select {in y; in w; in z; w+1}; 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/expansion.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a2ce296b3e011cf184967b167990e9dc1115f585d8be09ff4907dae94902685d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::select") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default) (role result-expression)) (relationships (expressionOperand (reference "w")))))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "w"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "y"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "z"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x")))))
    (reference (id (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "w")))))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "w"))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "w"))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "y"))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "z"))))) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x")))
      (supertype (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "w")))))
      (featured-by (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "y")))))
      (featured-by (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "z")))))
      (featured-by (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "+" (feature-reference "w" (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "w")))))) (literal (value (kind integer) (integer 1)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/expansion.md") (range (start 1 16) (end 1 40)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/expansion.md") (range (start 2 13) (end 2 14)) (probe (position 2 13))
    (reference (id (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x")))))
    )
  )
  (query (document "memory://snapshot/expansion.md") (range (start 2 42) (end 2 43)) (probe (position 2 42))
    (reference (id (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (named (kind kerml-feature) (name "x")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (named (kind parameter) (name "w")))))))
    )
  )
)
~~~
