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
# EXPECTED
~~~
semantic.unresolved_name 'A'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'A'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "PrivateImportTest"))) (name "PrivateImportTest") (declared-name "PrivateImportTest")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "PrivateImportTest::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "PrivateImportTest::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "PrivateImportTest::P1"))) (name "P1") (declared-name "P1")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "PrivateImportTest::P1::A"))) (name "A") (declared-name "A") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "PrivateImportTest::P2"))) (name "P2") (declared-name "P2")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "PrivateImportTest::P2::*"))) (name "*") (declared-name "*"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "PrivateImportTest::P3"))) (name "P3") (declared-name "P3")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "PrivateImportTest::P3::B"))) (name "B") (declared-name "B") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "PrivateImportTest::P4"))) (name "P4") (declared-name "P4")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "PrivateImportTest::P4::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "PrivateImportTest::P4::z1"))) (name "z1") (declared-name "z1") (declared (properties (ordered false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "PrivateImportTest::x"))) (name "x") (declared-name "x") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "PrivateImportTest::z"))) (name "z") (declared-name "z") (declared (properties (ordered false))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "PrivateImportTest::P4::z1"))) (to (node (document "d0") (qualified-name "PrivateImportTest::P1::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PrivateImportTest::x"))) (to (node (document "d0") (qualified-name "PrivateImportTest::P1::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PrivateImportTest::z"))) (to (node (document "d0") (qualified-name "PrivateImportTest::P3::B"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/private_import_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 17) (end 5 19))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 8 9) (end 8 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 20) (end 27 22))
      )
    )
  )
)
~~~
