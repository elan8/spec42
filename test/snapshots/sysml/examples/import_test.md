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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "import_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 20) (end 2 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 20) (end 3 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 20) (end 4 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 19) (end 5 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 23) (end 10 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 27) (end 13 30))
      )
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "960ac6c14c91fb3ed81156173668a8a1ed0c2e79738395308fb31dc5303979bc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ImportTest"))) (kind "package") (name "ImportTest") (declared-name "ImportTest") (range (start (line 0) (character 0)) (end (line 0) (character 381))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (kind "package") (name "Pkg1") (declared-name "Pkg1") (range (start (line 1) (character 4)) (end (line 1) (character 197))) (parent (node (document "d0") (qualified-name "ImportTest"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 5)) (end (line 3) (character 35))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg2::Pkg21::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 20)) (end (line 3) (character 31))))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 5)) (end (line 4) (character 34))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg211::*") (origin Import) (shape Namespace) (recursive true)) (import-range (start (line 4) (character 20)) (end (line 4) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::P12"))) (kind "part def") (name "P12") (declared-name "P12") (range (start (line 6) (character 8)) (end (line 6) (character 21))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::P211"))) (kind "import") (name "P211") (declared-name "P211") (range (start (line 2) (character 5)) (end (line 2) (character 46))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg2::Pkg21::Pkg211::P211") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 20)) (end (line 2) (character 45))))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::p11"))) (kind "part") (name "p11") (declared-name "p11") (range (start (line 5) (character 8)) (end (line 5) (character 32))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Pkg211::P211") (range (start (line 5) (character 19)) (end (line 5) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2"))) (kind "package") (name "Pkg2") (declared-name "Pkg2") (range (start (line 9) (character 4)) (end (line 9) (character 159))) (parent (node (document "d0") (qualified-name "ImportTest"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 8)) (end (line 10) (character 31))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 23)) (end (line 10) (character 27))))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21"))) (kind "package") (name "Pkg21") (declared-name "Pkg21") (range (start (line 11) (character 8)) (end (line 11) (character 102))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg2"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211"))) (kind "package") (name "Pkg211") (declared-name "Pkg211") (range (start (line 12) (character 9)) (end (line 12) (character 68))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind "part def") (name "P211") (declared-name "P211") (range (start (line 13) (character 10)) (end (line 13) (character 31))) (parent (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P12") (range (start (line 13) (character 27)) (end (line 13) (character 30)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Pkg2::Pkg21::*") (range (start (line 3) (character 20)) (end (line 3) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Pkg211::*") (range (start (line 4) (character 20)) (end (line 4) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::P211"))) (kind membershipImport) (ordinal 0)) (authored-target "Pkg2::Pkg21::Pkg211::P211") (range (start (line 2) (character 20)) (end (line 2) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::p11"))) (kind featureTyping) (ordinal 0)) (authored-target "Pkg211::P211") (range (start (line 5) (character 19)) (end (line 5) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Pkg1::*") (range (start (line 10) (character 23)) (end (line 10) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind specialization) (ordinal 0)) (authored-target "P12") (range (start (line 13) (character 27)) (end (line 13) (character 30))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
