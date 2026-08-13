# META
~~~ini
description=KerML Simple Tests: Circular
type=file
~~~
# SOURCE
~~~kerml
package Circular {
	class A { }
	feature a: A;
	alias Circ for Circular;
	package P {
		public import Circular::*;
	}
	
	feature x :> z;
	feature y :> x;
	feature z :> y;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/circular.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 1) (end 2 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 8 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 1) (end 9 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 10 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:33b40769c2b01f3ecc9bf9406b603b8c7dd94965ced45e5f4123464ed99ca427") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Circular"))))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Circular") (import (shape namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Circular")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular")))))
    (reference (id (source (node (document "memory://snapshot/circular.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Circular")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular")))))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (kind aliasBinding) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/circular.md") (range (start 3 16) (end 3 24)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (kind aliasBinding) (ordinal 0) (authored-target "Circular")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular")))))
  )
  (query (document "memory://snapshot/circular.md") (range (start 5 16) (end 5 27)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/circular.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Circular")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular")))))
  )
)
~~~
