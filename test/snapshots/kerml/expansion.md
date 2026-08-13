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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 13) (end 2 46))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:dd4a691d1574584940080c79df61d3840e0ad0de94c68b9950ca3720f8d52ba1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/expansion.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::select") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/expansion.md") (qualified-name "Expansion::x"))) (kind default-reference) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/expansion.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/expansion.md") (range (start 1 16) (end 1 40)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/expansion.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
  )
)
~~~
