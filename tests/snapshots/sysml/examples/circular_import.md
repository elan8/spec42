# META
~~~ini
description=SysML Example (Import Tests): CircularImport
type=file
~~~
# SOURCE
~~~sysml
package CircularImport {

	package P1 {
		public import P2::*;
		part def A;
	}
	package P2 {
		public import P1::*;
		part def B;
	}
	package Test1 {
		public import P1::*;
		part x: A;
		part y: B;
	}
	package Test2 {
		public import P2::*;
		part x: A;
		part y: B;
	}
	
	part x: P1::A;
	
	// The following should not fail.
	part y: P1::B;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/circular_import.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8c4b284f8e0a4d416e032c64662e080818082d0d65a9a506c140a2a4b8ce6af7") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "P1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P2") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P1") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "Test1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P1") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::x"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::y"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "Test2")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P2") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::x"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::y"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::x"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P1::A")))))
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::y"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P1::B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "P1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "Test1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "Test2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "P1::A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")))))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "P1::B")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::x"))) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::y"))) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::x"))) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::y"))) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::x"))) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::y"))) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::y"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")))
      (subtype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::x")) (scopes any))
      (subtype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::x")) (scopes any))
      (subtype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::x")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")))
      (subtype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::y")) (scopes any))
      (subtype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::y")) (scopes any))
      (subtype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::x")))
      (type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (source direct))
      (supertype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::y")))
      (type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (source direct))
      (supertype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::x")))
      (type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (source direct))
      (supertype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::y")))
      (type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (source direct))
      (supertype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::x")))
      (type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (source direct))
      (supertype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::y")))
      (type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (source direct))
      (supertype (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/circular_import.md") (range (start 3 16) (end 3 21)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "P1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 11 16) (end 11 21)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "Test1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 12 10) (end 12 11)) (probe (position 12 10))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::x"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 13 10) (end 13 11)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test1::y"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 16 16) (end 16 21)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (path (named (kind package) (name "CircularImport")) (named (kind package) (name "Test2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 17 10) (end 17 11)) (probe (position 17 10))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::x"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 18 10) (end 18 11)) (probe (position 18 10))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::Test2::y"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 21 9) (end 21 14)) (probe (position 21 9))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::x"))) (kind featureTyping) (ordinal 0) (authored-target "P1::A")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P1::A")))))
    )
  )
  (query (document "memory://snapshot/circular_import.md") (range (start 24 9) (end 24 14)) (probe (position 24 9))
    (reference (id (source (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::y"))) (kind featureTyping) (ordinal 0) (authored-target "P1::B")
      (outcome (status resolved) (target (node (document "memory://snapshot/circular_import.md") (qualified-name "CircularImport::P2::B")))))
    )
  )
)
~~~
