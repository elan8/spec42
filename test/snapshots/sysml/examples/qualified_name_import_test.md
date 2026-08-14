# META
~~~ini
description=SysML Example (Import Tests): QualifiedNameImportTest
type=file
~~~
# SOURCE
~~~sysml
package QualifiedNameImportTest {
	package P1 {
		part def A;
	}
	package P2 {
		package P2a {
			public import P1::*;
		}
		// The following should not fail.
		// A is a member of P2a because of the import.
		part x: P2a::A;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/qualified_name_import_test.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8916919ba87bb39134c9acc4697ee973629451e91945aca26f0c4a471e292868") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P1::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P2::P2a"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/qualified_name_import_test.md") (path (name "QualifiedNameImportTest") (name "P2") (name "P2a") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "P1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P2::x"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P2a::A"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/qualified_name_import_test.md") (path (name "QualifiedNameImportTest") (name "P2") (name "P2a") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P1")))))
    (reference (id (source (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P2::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "P2a::A")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P1::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P2::x"))) (target (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P1::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P2::x"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/qualified_name_import_test.md") (range (start 6 17) (end 6 22)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/qualified_name_import_test.md") (path (name "QualifiedNameImportTest") (name "P2") (name "P2a") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "P1")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P1")))))
  )
  (query (document "memory://snapshot/qualified_name_import_test.md") (range (start 10 10) (end 10 16)) (probe (position 10 10))
    (reference (id (source (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P2::x"))) (kind featureTyping) (ordinal 0) (authored-target "P2a::A")
      (outcome (status resolved) (target (node (document "memory://snapshot/qualified_name_import_test.md") (qualified-name "QualifiedNameImportTest::P1::A")))))
  )
)
~~~
