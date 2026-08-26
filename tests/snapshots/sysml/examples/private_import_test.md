# META
~~~ini
description=SysML Example (Import Tests): PrivateImportTest
type=file
~~~
# SOURCE
~~~sysml
package PrivateImportTest {
	package P1 {
		part def A;
	}
	package P2 {
		private import P1::*;
	}

	part x: P1::A;
	
	public import P2::*;
	// This should fail.
	// A is not visible, because the import in P2 is private.
	// part y: A;
	// part y1: P2::A;
	
	package P3 {
		part def B;
	}
	
	private import P3::*;
	
	// This should not fail.
	// Private import only restricts visibility outside the package.
	part z: B;
	
	package P4 {
		public import all P2::*;
		
		// This should not fail because "import all" overrides private import.
		part z1: A;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/private_import_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 5 17) (end 5 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 11) (end 30 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:967849580a52ec99d90195187a1cd1d0e3f21d87c1c34a526751b0de9560b2b2") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P2") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "P3") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "P1") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P4"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (named (kind package) (name "P4")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P2") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P4::z1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::x"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P1::A")))))
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::z"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P2")))))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P3")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3")))))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1")))))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (named (kind package) (name "P4")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P2")))))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P4::z1"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "P1::A")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A")))))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::z"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::x"))) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::z"))) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::z"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A")))
      (subtype (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::x")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B")))
      (subtype (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::z")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::x")))
      (type (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A")) (source direct))
      (supertype (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::z")))
      (type (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B")) (source direct))
      (supertype (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/private_import_test.md") (range (start 10 15) (end 10 20)) (probe (position 10 15))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P2")))))
    )
  )
  (query (document "memory://snapshot/private_import_test.md") (range (start 20 16) (end 20 21)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "P3")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3")))))
    )
  )
  (query (document "memory://snapshot/private_import_test.md") (range (start 5 17) (end 5 22)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (named (kind package) (name "P2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1")))))
    )
  )
  (query (document "memory://snapshot/private_import_test.md") (range (start 27 16) (end 27 25)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (path (named (kind package) (name "PrivateImportTest")) (named (kind package) (name "P4")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "P2")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P2")))))
    )
  )
  (query (document "memory://snapshot/private_import_test.md") (range (start 30 11) (end 30 12)) (probe (position 30 11))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P4::z1"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/private_import_test.md") (range (start 8 9) (end 8 14)) (probe (position 8 9))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::x"))) (kind featureTyping) (ordinal 0) (authored-target "P1::A")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P1::A")))))
    )
  )
  (query (document "memory://snapshot/private_import_test.md") (range (start 24 9) (end 24 10)) (probe (position 24 9))
    (reference (id (source (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::z"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/private_import_test.md") (qualified-name "PrivateImportTest::P3::B")))))
    )
  )
)
~~~
