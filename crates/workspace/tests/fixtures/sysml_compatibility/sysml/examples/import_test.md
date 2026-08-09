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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ImportTest"))) (name "ImportTest") (declared-name "ImportTest")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (name "Pkg1") (declared-name "Pkg1")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "ImportTest::Pkg1::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "ImportTest::Pkg1::*#import"))) (name "*") (declared-name "*"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "ImportTest::Pkg1::P12"))) (name "P12") (declared-name "P12") (declared))
            (element (kind "import") (id (node (document "d0") (qualified-name "ImportTest::Pkg1::P211"))) (name "P211") (declared-name "P211"))
            (element (kind "part") (id (node (document "d0") (qualified-name "ImportTest::Pkg1::p11"))) (name "p11") (declared-name "p11") (declared (properties (composite true) (reference false) (ordered false))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "ImportTest::Pkg2"))) (name "Pkg2") (declared-name "Pkg2")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "ImportTest::Pkg2::*"))) (name "*") (declared-name "*"))
            (element (kind "package") (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21"))) (name "Pkg21") (declared-name "Pkg21")
              (contains
                (element (kind "package") (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211"))) (name "Pkg211") (declared-name "Pkg211")
                  (contains
                    (element (kind "part def") (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (name "P211") (declared-name "P211") (declared))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (to (node (document "d0") (qualified-name "ImportTest::Pkg1::P12"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
