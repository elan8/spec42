# META
~~~ini
description=SysML Example (Simple Tests): ImportTest
type=file
~~~
# SOURCE
~~~sysml
package ImportTest {
    package Pkg1 {
    	private import Pkg2::Pkg21::Pkg211::P211;
    	private import Pkg2::Pkg21::*;
    	private import Pkg211::*::**;
        part p11 : Pkg211::P211;
        part def P12;
    }

    package Pkg2 {
        private import Pkg1::*;
        package Pkg21 {
        	package Pkg211 {
        		part def P211 :> P12;
        	}
        }
    }
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,ColonColon,StarStar,Semicolon,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ImportTest'
    (package_def 'Pkg1'
      (import_decl private 'Pkg2::Pkg21::Pkg211::P211')
      (import_decl private 'Pkg2::Pkg21::*')
      (import_decl private 'Pkg211::*::**')
      (part_usage 'p11' : 'Pkg211::P211')
      (part_def 'P12'))
    (package_def 'Pkg2'
      (import_decl private 'Pkg1::*')
      (package_def 'Pkg21'
        (package_def 'Pkg211'
          (part_def 'P211' :> 'P12'))))))
~~~
# FORMAT
~~~sysml
package ImportTest {
    package Pkg1 {
        private import Pkg2::Pkg21::Pkg211::P211;
        private import Pkg2::Pkg21::*;
        private import Pkg211::*::**;
        part p11 : Pkg211::P211;
        part def P12;
    }

    package Pkg2 {
        private import Pkg1::*;
        package Pkg21 {
            package Pkg211 {
                part def P211 :> P12;
            }
        }
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
    (package 'ImportTest'
      (package 'Pkg1'
        (membership_import private -> 'ImportTest::Pkg2::Pkg21::Pkg211::P211'[part_def])
        (namespace_import private -> 'ImportTest::Pkg2::Pkg21'[package])
        (namespace_import private recursive -> 'ImportTest::Pkg2::Pkg21::Pkg211'[package])
        (part_usage 'p11' : 'ImportTest::Pkg2::Pkg21::Pkg211::P211'[part_def])
        (part_def 'P12'))
      (package 'Pkg2'
        (namespace_import private -> 'ImportTest::Pkg1'[package])
        (package 'Pkg21'
          (package 'Pkg211'
            (part_def 'P211' :> 'ImportTest::Pkg1::P12'[part_def])))))))
~~~
