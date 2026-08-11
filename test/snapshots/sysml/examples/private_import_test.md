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
  (document "private_import_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 17) (end 5 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 20) (end 27 22))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1e03689f501fa5fdfca0ac5aae07404a8bdf3575d74328c39d6a9c7d064eea08") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "PrivateImportTest"))) (kind "package") (name "PrivateImportTest") (declared-name "PrivateImportTest"))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Import) (visibility "public") (import (reference "P2::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "P3::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P1"))) (kind "package") (name "P1") (declared-name "P1") (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P1::A"))) (kind "part def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "PrivateImportTest::P1"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P2"))) (kind "package") (name "P2") (declared-name "P2") (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "PrivateImportTest::P2"))) (authored (membership (kind Import) (visibility "private") (import (reference "P1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P3"))) (kind "package") (name "P3") (declared-name "P3") (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P3::B"))) (kind "part def") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "PrivateImportTest::P3"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P4"))) (kind "package") (name "P4") (declared-name "P4") (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P4::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "PrivateImportTest::P4"))) (authored (membership (kind Import) (visibility "public") (import (reference "P2::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P4::z1"))) (kind "part") (name "z1") (declared-name "z1") (parent (node (document "d0") (qualified-name "PrivateImportTest::P4"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::x"))) (kind "part") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P1::A")))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::z"))) (kind "part") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "B")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P2::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P2")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "P3::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P3")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::P2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::P4::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P2::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::P4::z1"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::x"))) (kind featureTyping) (ordinal 0)) (authored-target "P1::A") (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P1::A")))))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::z"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P3::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PrivateImportTest::x"))) (target (node (document "d0") (qualified-name "PrivateImportTest::P1::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PrivateImportTest::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PrivateImportTest::z"))) (target (node (document "d0") (qualified-name "PrivateImportTest::P3::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PrivateImportTest::z"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 24 9) (end 24 10)) (probe (position 24 9))
      (reference
        (source (document "d0") (qualified-name "PrivateImportTest::z"))
        (kind featureTyping) (ordinal 0) (authored-target "B")
        (range (start 24 9) (end 24 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PrivateImportTest::P3::B") (range (start 17 2) (end 17 13)))
        )
      )
    )
    (query (range (start 30 11) (end 30 12)) (probe (position 30 11))
      (reference
        (source (document "d0") (qualified-name "PrivateImportTest::P4::z1"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 30 11) (end 30 12))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 17) (end 5 19)) (probe (position 5 17))
      (reference
        (source (document "d0") (qualified-name "PrivateImportTest::P2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P1::*")
        (range (start 5 17) (end 5 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 15) (end 10 17)) (probe (position 10 15))
      (reference
        (source (document "d0") (qualified-name "PrivateImportTest::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P2::*")
        (range (start 10 15) (end 10 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PrivateImportTest::P2") (range (start 4 1) (end 4 40)))
        )
      )
    )
    (query (range (start 20 16) (end 20 18)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "PrivateImportTest::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "P3::*")
        (range (start 20 16) (end 20 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PrivateImportTest::P3") (range (start 16 1) (end 16 30)))
        )
      )
    )
    (query (range (start 27 20) (end 27 22)) (probe (position 27 20))
      (reference
        (source (document "d0") (qualified-name "PrivateImportTest::P4::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P2::*")
        (range (start 27 20) (end 27 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 9) (end 8 14)) (probe (position 8 9))
      (reference
        (source (document "d0") (qualified-name "PrivateImportTest::x"))
        (kind featureTyping) (ordinal 0) (authored-target "P1::A")
        (range (start 8 9) (end 8 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PrivateImportTest::P1::A") (range (start 2 2) (end 2 13)))
        )
      )
    )
  )
)
~~~
