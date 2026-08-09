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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Imports"))) (name "Imports") (declared-name "Imports")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "Imports::P"))) (name "P") (declared-name "P")
          (contains
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::P::A"))) (name "A") (declared-name "A"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::P::B"))) (name "B") (declared-name "B"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::P::C"))) (name "C") (declared-name "C"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Imports::Q"))) (name "Q") (declared-name "Q")
          (contains
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::Q::A"))) (name "A") (declared-name "A"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::Q::D"))) (name "D") (declared-name "D"))
            (element (kind "package") (id (node (document "d0") (qualified-name "Imports::Q::Q1"))) (name "Q1") (declared-name "Q1")
              (contains
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::Q::Q1::D"))) (name "D") (declared-name "D"))
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::Q::Q1::E"))) (name "E") (declared-name "E"))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "Imports::Q::Q2"))) (name "Q2") (declared-name "Q2")
              (contains
                (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::Q::Q2::F"))) (name "F") (declared-name "F"))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Imports::R"))) (name "R") (declared-name "R")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Imports::R::*"))) (name "*") (declared-name "*"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Imports::S"))) (name "S") (declared-name "S")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Imports::S::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "Imports::S::Q"))) (name "Q") (declared-name "Q"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::S::X"))) (name "X") (declared-name "X"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::S::Y"))) (name "Y") (declared-name "Y"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::S::Z"))) (name "Z") (declared-name "Z"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Imports::S1"))) (name "S1") (declared-name "S1")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Imports::S1::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "Imports::S1::*#import"))) (name "*") (declared-name "*"))
            (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Imports::S1::X"))) (name "X") (declared-name "X"))
          )
        )
      )
    )
  )
  (relationships
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
  (document "kerml/imports.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 16 3) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 2) (end 26 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 31 2) (end 31 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 32 2) (end 32 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 41 2) (end 41 21))
      )
    )
  )
)
~~~
