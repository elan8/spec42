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
  (document "qualified_name_import_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 17) (end 6 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 10) (end 10 16))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4d0ce82c4087280b67501dc8aa760c1f0d2500d14982c0694e7d8ccb1d2e6ac6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "QualifiedNameImportTest"))) (kind "package") (name "QualifiedNameImportTest") (declared-name "QualifiedNameImportTest") (range (start (line 0) (character 0)) (end (line 0) (character 230))))
    (element (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P1"))) (kind "package") (name "P1") (declared-name "P1") (range (start (line 1) (character 1)) (end (line 1) (character 30))) (parent (node (document "d0") (qualified-name "QualifiedNameImportTest"))))
    (element (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P1::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 2) (character 2)) (end (line 2) (character 13))) (parent (node (document "d0") (qualified-name "QualifiedNameImportTest::P1"))))
    (element (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2"))) (kind "package") (name "P2") (declared-name "P2") (range (start (line 4) (character 1)) (end (line 4) (character 163))) (parent (node (document "d0") (qualified-name "QualifiedNameImportTest"))))
    (element (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::P2a"))) (kind "package") (name "P2a") (declared-name "P2a") (range (start (line 5) (character 2)) (end (line 5) (character 43))) (parent (node (document "d0") (qualified-name "QualifiedNameImportTest::P2"))))
    (element (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::P2a::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 3)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::P2a"))) (authored (membership (kind Import) (visibility "public") (import (reference "P1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 17)) (end (line 6) (character 19))))))
    (element (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::x"))) (kind "part") (name "x") (declared-name "x") (range (start (line 10) (character 2)) (end (line 10) (character 17))) (parent (node (document "d0") (qualified-name "QualifiedNameImportTest::P2"))) (authored (membership (kind Feature)) (relationships (typing (reference "P2a::A") (range (start (line 10) (character 10)) (end (line 10) (character 16)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::P2a::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P1::*") (range (start (line 6) (character 17)) (end (line 6) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::x"))) (kind featureTyping) (ordinal 0)) (authored-target "P2a::A") (range (start (line 10) (character 10)) (end (line 10) (character 16))) (outcome (status unresolved)))
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
  (document "d0"
    (query (range (start 6 17) (end 6 19)) (probe (position 6 17))
      (reference
        (source (document "d0") (qualified-name "QualifiedNameImportTest::P2::P2a::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P1::*")
        (range (start 6 17) (end 6 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 10) (end 10 16)) (probe (position 10 10))
      (reference
        (source (document "d0") (qualified-name "QualifiedNameImportTest::P2::x"))
        (kind featureTyping) (ordinal 0) (authored-target "P2a::A")
        (range (start 10 10) (end 10 16))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
