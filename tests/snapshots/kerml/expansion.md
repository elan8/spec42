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
        (severity information)
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:dd4a691d1574584940080c79df61d3840e0ad0de94c68b9950ca3720f8d52ba1") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::select") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/expansion.md") (path (named (kind package) (name "Expansion")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (target (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
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
    (reference (id (source (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x")))))
    )
  )
)
~~~
