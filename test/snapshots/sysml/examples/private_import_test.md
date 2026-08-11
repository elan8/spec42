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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
LineComment,
LineComment,
LineComment,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
LineComment,
KwPart,Ident,Colon,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,KwAll,Ident,ColonColon,Star,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'PrivateImportTest'
    (package_def 'P1'
      (part_def 'A'))
    (package_def 'P2'
      (import_decl private 'P1::*'))
    (part_usage 'x' : 'P1::A')
    (import_decl public 'P2::*')
    (line_comment)
    (line_comment)
    (line_comment)
    (line_comment)
    (package_def 'P3'
      (part_def 'B'))
    (import_decl private 'P3::*')
    (line_comment)
    (line_comment)
    (part_usage 'z' : 'B')
    (package_def 'P4'
      (import_decl public all 'P2::*')
      (line_comment)
      (part_usage 'z1' : 'A'))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'A'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'A'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4575fe6bf10a131244ddcf5ecc7e8001139c79f9fa9364be232ad07ff01f35cb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "PrivateImportTest"))) (kind "package") (name "PrivateImportTest") (declared-name "PrivateImportTest") (range (start (line 0) (character 0)) (end (line 0) (character 559))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 1)) (end (line 10) (character 21))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Import) (visibility "public") (import (reference "P2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 15)) (end (line 10) (character 17))))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 20) (character 1)) (end (line 20) (character 22))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "P3::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 20) (character 16)) (end (line 20) (character 18))))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P1"))) (kind "package") (name "P1") (declared-name "P1") (range (start (line 1) (character 1)) (end (line 1) (character 30))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P1::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 2) (character 2)) (end (line 2) (character 13))) (parent (node (document "d0") (qualified-name "PrivateImportTest::P1"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P2"))) (kind "package") (name "P2") (declared-name "P2") (range (start (line 4) (character 1)) (end (line 4) (character 40))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 2)) (end (line 5) (character 23))) (parent (node (document "d0") (qualified-name "PrivateImportTest::P2"))) (authored (membership (kind Import) (visibility "private") (import (reference "P1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 17)) (end (line 5) (character 19))))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P3"))) (kind "package") (name "P3") (declared-name "P3") (range (start (line 16) (character 1)) (end (line 16) (character 30))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P3::B"))) (kind "part def") (name "B") (declared-name "B") (range (start (line 17) (character 2)) (end (line 17) (character 13))) (parent (node (document "d0") (qualified-name "PrivateImportTest::P3"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P4"))) (kind "package") (name "P4") (declared-name "P4") (range (start (line 26) (character 1)) (end (line 26) (character 133))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P4::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 27) (character 2)) (end (line 27) (character 26))) (parent (node (document "d0") (qualified-name "PrivateImportTest::P4"))) (authored (membership (kind Import) (visibility "public") (import (reference "P2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 27) (character 20)) (end (line 27) (character 22))))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::P4::z1"))) (kind "part") (name "z1") (declared-name "z1") (range (start (line 30) (character 2)) (end (line 30) (character 13))) (parent (node (document "d0") (qualified-name "PrivateImportTest::P4"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 30) (character 11)) (end (line 30) (character 12)))))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::x"))) (kind "part") (name "x") (declared-name "x") (range (start (line 8) (character 1)) (end (line 8) (character 15))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P1::A") (range (start (line 8) (character 9)) (end (line 8) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "PrivateImportTest::z"))) (kind "part") (name "z") (declared-name "z") (range (start (line 24) (character 1)) (end (line 24) (character 11))) (parent (node (document "d0") (qualified-name "PrivateImportTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range (start (line 24) (character 9)) (end (line 24) (character 10)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P2::*") (range (start (line 10) (character 15)) (end (line 10) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P2")))))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "P3::*") (range (start (line 20) (character 16)) (end (line 20) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P3")))))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::P2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P1::*") (range (start (line 5) (character 17)) (end (line 5) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::P4::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P2::*") (range (start (line 27) (character 20)) (end (line 27) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::P4::z1"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 30) (character 11)) (end (line 30) (character 12))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::x"))) (kind featureTyping) (ordinal 0)) (authored-target "P1::A") (range (start (line 8) (character 9)) (end (line 8) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P1::A")))))
    (reference (id (source (node (document "d0") (qualified-name "PrivateImportTest::z"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range (start (line 24) (character 9)) (end (line 24) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PrivateImportTest::P3::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PrivateImportTest::x"))) (target (node (document "d0") (qualified-name "PrivateImportTest::P1::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PrivateImportTest::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PrivateImportTest::z"))) (target (node (document "d0") (qualified-name "PrivateImportTest::P3::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PrivateImportTest::z"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
