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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
CloseCurly,
LineComment,
LineComment,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'QualifiedNameImportTest'
    (package_def 'P1'
      (part_def 'A'))
    (package_def 'P2'
      (package_def 'P2a'
        (import_decl public 'P1::*'))
      (line_comment)
      (line_comment)
      (part_usage 'x' : 'P2a::A'))))
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
        part x : P2a::A;
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package 'QualifiedNameImportTest'
      (package 'P1'
        (part_def 'A'))
      (package 'P2'
        (package 'P2a'
          (namespace_import public -> 'QualifiedNameImportTest::P1'[package]))
        (part_usage 'x' : 'QualifiedNameImportTest::P1::A'[part_def])))))
~~~
