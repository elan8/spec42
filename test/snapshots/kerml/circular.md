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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:33b40769c2b01f3ecc9bf9406b603b8c7dd94965ced45e5f4123464ed99ca427") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Circular")))))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular.md") (path (named (kind package) (name "Circular")) (named (kind package) (name "P")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Circular") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "z")))))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "x")))))
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "y")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Circular")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular")))))
    (reference (id (source (node (document "memory://snapshot/circular.md") (path (named (kind package) (name "Circular")) (named (kind package) (name "P")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Circular")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular")))))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A")))))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x"))) (kind subsetting) (ordinal 0))
      (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z")))))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y"))) (kind subsetting) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x")))))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z"))) (kind subsetting) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y")))))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::Circ"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::a"))) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x"))) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y"))) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z"))) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A")))
      (subtype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::a")))
      (type (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A")) (source direct))
      (supertype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x"))) (cyclic true)
      (supertype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y")) (scopes any feature))
      (supertype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z")) (scopes any feature))
      (subtype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y"))) (cyclic true)
      (supertype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x")) (scopes any feature))
      (supertype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z")) (scopes any feature))
      (subtype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z"))) (cyclic true)
      (supertype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x")) (scopes any feature))
      (supertype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y")) (scopes any feature))
      (subtype (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x")) (scopes any feature))
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
  )
  (query (document "memory://snapshot/circular.md") (range (start 5 16) (end 5 27)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/circular.md") (path (named (kind package) (name "Circular")) (named (kind package) (name "P")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Circular")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular")))))
    )
  )
  (query (document "memory://snapshot/circular.md") (range (start 2 12) (end 2 13)) (probe (position 2 12))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::A")))))
    )
  )
  (query (document "memory://snapshot/circular.md") (range (start 8 14) (end 8 15)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x"))) (kind subsetting) (ordinal 0) (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z")))))
    )
  )
  (query (document "memory://snapshot/circular.md") (range (start 9 14) (end 9 15)) (probe (position 9 14))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y"))) (kind subsetting) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::x")))))
    )
  )
  (query (document "memory://snapshot/circular.md") (range (start 10 14) (end 10 15)) (probe (position 10 14))
    (reference (id (source (node (document "memory://snapshot/circular.md") (qualified-name "Circular::z"))) (kind subsetting) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular.md") (qualified-name "Circular::y")))))
    )
  )
)
~~~
