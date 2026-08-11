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
  (document "circular_import.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 10) (end 12 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 10) (end 13 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 10) (end 17 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 10) (end 18 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 9) (end 24 14))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c737ea67cef5a2ebf5d4ee93708e32cf1dc0c8f5daea844419f1cd7f2709f5f1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CircularImport"))) (kind "package") (name "CircularImport") (declared-name "CircularImport") (range (start (line 0) (character 0)) (end (line 0) (character 346))))
    (element (id (node (document "d0") (qualified-name "CircularImport::P1"))) (kind "package") (name "P1") (declared-name "P1") (range (start (line 2) (character 1)) (end (line 2) (character 53))) (parent (node (document "d0") (qualified-name "CircularImport"))))
    (element (id (node (document "d0") (qualified-name "CircularImport::P1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 2)) (end (line 3) (character 22))) (parent (node (document "d0") (qualified-name "CircularImport::P1"))) (authored (membership (kind Import) (visibility "public") (import (reference "P2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 18))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::P1::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 4) (character 2)) (end (line 4) (character 13))) (parent (node (document "d0") (qualified-name "CircularImport::P1"))))
    (element (id (node (document "d0") (qualified-name "CircularImport::P2"))) (kind "package") (name "P2") (declared-name "P2") (range (start (line 6) (character 1)) (end (line 6) (character 53))) (parent (node (document "d0") (qualified-name "CircularImport"))))
    (element (id (node (document "d0") (qualified-name "CircularImport::P2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 2)) (end (line 7) (character 22))) (parent (node (document "d0") (qualified-name "CircularImport::P2"))) (authored (membership (kind Import) (visibility "public") (import (reference "P1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 18))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::P2::B"))) (kind "part def") (name "B") (declared-name "B") (range (start (line 8) (character 2)) (end (line 8) (character 13))) (parent (node (document "d0") (qualified-name "CircularImport::P2"))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test1"))) (kind "package") (name "Test1") (declared-name "Test1") (range (start (line 10) (character 1)) (end (line 10) (character 68))) (parent (node (document "d0") (qualified-name "CircularImport"))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 11) (character 2)) (end (line 11) (character 22))) (parent (node (document "d0") (qualified-name "CircularImport::Test1"))) (authored (membership (kind Import) (visibility "public") (import (reference "P1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 18))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test1::x"))) (kind "part") (name "x") (declared-name "x") (range (start (line 12) (character 2)) (end (line 12) (character 12))) (parent (node (document "d0") (qualified-name "CircularImport::Test1"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 12) (character 10)) (end (line 12) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test1::y"))) (kind "part") (name "y") (declared-name "y") (range (start (line 13) (character 2)) (end (line 13) (character 12))) (parent (node (document "d0") (qualified-name "CircularImport::Test1"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range (start (line 13) (character 10)) (end (line 13) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test2"))) (kind "package") (name "Test2") (declared-name "Test2") (range (start (line 15) (character 1)) (end (line 15) (character 68))) (parent (node (document "d0") (qualified-name "CircularImport"))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 2)) (end (line 16) (character 22))) (parent (node (document "d0") (qualified-name "CircularImport::Test2"))) (authored (membership (kind Import) (visibility "public") (import (reference "P2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 18))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test2::x"))) (kind "part") (name "x") (declared-name "x") (range (start (line 17) (character 2)) (end (line 17) (character 12))) (parent (node (document "d0") (qualified-name "CircularImport::Test2"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 17) (character 10)) (end (line 17) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::Test2::y"))) (kind "part") (name "y") (declared-name "y") (range (start (line 18) (character 2)) (end (line 18) (character 12))) (parent (node (document "d0") (qualified-name "CircularImport::Test2"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range (start (line 18) (character 10)) (end (line 18) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::x"))) (kind "part") (name "x") (declared-name "x") (range (start (line 21) (character 1)) (end (line 21) (character 15))) (parent (node (document "d0") (qualified-name "CircularImport"))) (authored (membership (kind Feature)) (relationships (typing (reference "P1::A") (range (start (line 21) (character 9)) (end (line 21) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "CircularImport::y"))) (kind "part") (name "y") (declared-name "y") (range (start (line 24) (character 1)) (end (line 24) (character 15))) (parent (node (document "d0") (qualified-name "CircularImport"))) (authored (membership (kind Feature)) (relationships (typing (reference "P1::B") (range (start (line 24) (character 9)) (end (line 24) (character 14)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::P1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P2::*") (range (start (line 3) (character 16)) (end (line 3) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::P2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P1::*") (range (start (line 7) (character 16)) (end (line 7) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::Test1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P1::*") (range (start (line 11) (character 16)) (end (line 11) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::Test1::x"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 12) (character 10)) (end (line 12) (character 11))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::Test1::y"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range (start (line 13) (character 10)) (end (line 13) (character 11))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::Test2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P2::*") (range (start (line 16) (character 16)) (end (line 16) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::Test2::x"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 17) (character 10)) (end (line 17) (character 11))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::Test2::y"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range (start (line 18) (character 10)) (end (line 18) (character 11))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::x"))) (kind featureTyping) (ordinal 0)) (authored-target "P1::A") (range (start (line 21) (character 9)) (end (line 21) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CircularImport::P1::A")))))
    (reference (id (source (node (document "d0") (qualified-name "CircularImport::y"))) (kind featureTyping) (ordinal 0)) (authored-target "P1::B") (range (start (line 24) (character 9)) (end (line 24) (character 14))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CircularImport::x"))) (target (node (document "d0") (qualified-name "CircularImport::P1::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CircularImport::x"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
