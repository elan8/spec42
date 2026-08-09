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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CircularImport'
    (package_def 'P1'
      (import_decl public 'P2::*')
      (part_def 'A'))
    (package_def 'P2'
      (import_decl public 'P1::*')
      (part_def 'B'))
    (package_def 'Test1'
      (import_decl public 'P1::*')
      (part_usage 'x' : 'A')
      (part_usage 'y' : 'B'))
    (package_def 'Test2'
      (import_decl public 'P2::*')
      (part_usage 'x' : 'A')
      (part_usage 'y' : 'B'))
    (part_usage 'x' : 'P1::A')
    (line_comment)
    (part_usage 'y' : 'P1::B')))
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
        part x : A;
        part y : B;
    }
    package Test2 {
        public import P2::*;
        part x : A;
        part y : B;
    }

    part x : P1::A;

    // The following should not fail.
    part y : P1::B;
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
    (package 'CircularImport'
      (package 'P1'
        (namespace_import public -> 'CircularImport::P2'[package])
        (part_def 'A'))
      (package 'P2'
        (namespace_import public -> 'CircularImport::P1'[package])
        (part_def 'B'))
      (package 'Test1'
        (namespace_import public -> 'CircularImport::P1'[package])
        (part_usage 'x' : 'CircularImport::P1::A'[part_def])
        (part_usage 'y' : 'CircularImport::P2::B'[part_def]))
      (package 'Test2'
        (namespace_import public -> 'CircularImport::P2'[package])
        (part_usage 'x' : 'CircularImport::P1::A'[part_def])
        (part_usage 'y' : 'CircularImport::P2::B'[part_def]))
      (part_usage 'x' : 'CircularImport::P1::A'[part_def])
      (part_usage 'y' : 'CircularImport::P2::B'[part_def]))))
~~~
