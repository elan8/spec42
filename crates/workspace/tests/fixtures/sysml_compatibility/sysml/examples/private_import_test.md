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

    part x : P1::A;

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
    part z : B;

    package P4 {
        public import all P2::*;

        // This should not fail because "import all" overrides private import.
        part z1 : A;
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
(model
  (namespace
    (package 'PrivateImportTest'
      (package 'P1'
        (part_def 'A'))
      (package 'P2'
        (namespace_import private -> 'PrivateImportTest::P1'[package]))
      (part_usage 'x' : 'PrivateImportTest::P1::A'[part_def])
      (namespace_import public -> 'PrivateImportTest::P2'[package])
      (package 'P3'
        (part_def 'B'))
      (namespace_import private -> 'PrivateImportTest::P3'[package])
      (part_usage 'z' : 'PrivateImportTest::P3::B'[part_def])
      (package 'P4'
        (namespace_import public all -> 'PrivateImportTest::P2'[package])
        (part_usage 'z1' : 'A'[unresolved])))))
~~~
