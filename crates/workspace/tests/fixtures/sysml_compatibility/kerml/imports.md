# META
~~~ini
description=KerML Simple Tests: Imports
type=file
~~~
# SOURCE
~~~kerml
package Imports {

	package P {
		class A;
		class B;
		class C;
	}
	
	package Q {
		class A;
		class D {
			class E;
		}
		package Q1 {
			class D;
			class E;
			private package Q1a {
				class G;
			}
		}
		package Q2 {
			class F;
		}
	}
	
	package R {
		public import Q::*;
	}

	
	package S {
		public import P::*;
		public import Q::**;
		
		class X :> A;
		class Y :> D;
		class Z :> F;
	}
	
	package S1 {
		public import P::*;
		public import R::*;
		
		class X :> A;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
KwClass,Ident,OpenCurly,
KwClass,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwPrivate,KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwClass,Ident,ColonGt,Ident,Semicolon,
KwClass,Ident,ColonGt,Ident,Semicolon,
KwClass,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Imports'
    (package_def 'P'
      (class_def 'A')
      (class_def 'B')
      (class_def 'C'))
    (package_def 'Q'
      (class_def 'A')
      (class_def 'D'
        (class_def 'E'))
      (package_def 'Q1'
        (class_def 'D')
        (class_def 'E')
        (package_def private 'Q1a'
          (class_def 'G')))
      (package_def 'Q2'
        (class_def 'F')))
    (package_def 'R'
      (import_decl public 'Q::*'))
    (package_def 'S'
      (import_decl public 'P::*')
      (import_decl public 'Q::**')
      (class_def 'X' :> 'A')
      (class_def 'Y' :> 'D')
      (class_def 'Z' :> 'F'))
    (package_def 'S1'
      (import_decl public 'P::*')
      (import_decl public 'R::*')
      (class_def 'X' :> 'A'))))
~~~
# FORMAT
~~~sysml
package Imports {
    package P {
        class A;
        class B;
        class C;
    }

    package Q {
        class A;
        class D {
            class E;
        }
        package Q1 {
            class D;
            class E;
            private package Q1a {
                class G;
            }
        }
        package Q2 {
            class F;
        }
    }

    package R {
        public import Q::*;
    }

    package S {
        public import P::*;
        public import Q::**;

        class X :> A;
        class Y :> D;
        class Z :> F;
    }

    package S1 {
        public import P::*;
        public import R::*;

        class X :> A;
    }
}
~~~
# EXPECTED
~~~
semantic.ambiguous_name 'A'
semantic.ambiguous_name 'D'
semantic.ambiguous_name 'A'
~~~
# PROBLEMS
~~~
semantic.ambiguous_name 'A'
semantic.ambiguous_name 'D'
semantic.ambiguous_name 'A'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Imports'
      (package 'P'
        (class_def 'A')
        (class_def 'B')
        (class_def 'C'))
      (package 'Q'
        (class_def 'A')
        (class_def 'D'
          (class_def 'E'))
        (package 'Q1'
          (class_def 'D')
          (class_def 'E')
          (package 'Q1a'
            (class_def 'G')))
        (package 'Q2'
          (class_def 'F')))
      (package 'R'
        (namespace_import public -> 'Imports::Q'[package]))
      (package 'S'
        (namespace_import public -> 'Imports::P'[package])
        (membership_import public recursive -> 'Imports::Q'[package])
        (class_def 'X' :> 'A'[unresolved])
        (class_def 'Y' :> 'D'[unresolved])
        (class_def 'Z' :> 'Imports::Q::Q2::F'[class_def]))
      (package 'S1'
        (namespace_import public -> 'Imports::P'[package])
        (namespace_import public -> 'Imports::R'[package])
        (class_def 'X' :> 'A'[unresolved])))))
~~~
