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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ac49382b186853d45439cc023bebec1edddaf15371002a768dc4245ec60fa309") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ImportTest"))) (kind "package") (name "ImportTest") (declared-name "ImportTest"))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (kind "package") (name "Pkg1") (declared-name "Pkg1") (parent (node (document "d0") (qualified-name "ImportTest"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg2::Pkg21::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg211::*") (origin Import) (shape Namespace) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::P12"))) (kind "part def") (name "P12") (declared-name "P12") (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::P211"))) (kind "import") (name "P211") (declared-name "P211") (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg2::Pkg21::Pkg211::P211") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg1::p11"))) (kind "part") (name "p11") (declared-name "p11") (parent (node (document "d0") (qualified-name "ImportTest::Pkg1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Pkg211::P211")))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2"))) (kind "package") (name "Pkg2") (declared-name "Pkg2") (parent (node (document "d0") (qualified-name "ImportTest"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ImportTest::Pkg2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Pkg1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21"))) (kind "package") (name "Pkg21") (declared-name "Pkg21") (parent (node (document "d0") (qualified-name "ImportTest::Pkg2"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211"))) (kind "package") (name "Pkg211") (declared-name "Pkg211") (parent (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21"))))
    (element (id (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind "part def") (name "P211") (declared-name "P211") (parent (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P12")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Pkg2::Pkg21::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Pkg211::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::P211"))) (kind membershipImport) (ordinal 0)) (authored-target "Pkg2::Pkg21::Pkg211::P211") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg1::p11"))) (kind featureTyping) (ordinal 0)) (authored-target "Pkg211::P211") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Pkg1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))) (kind specialization) (ordinal 0)) (authored-target "P12") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 13 27) (end 13 30)) (probe (position 13 27))
      (reference
        (source (document "d0") (qualified-name "ImportTest::Pkg2::Pkg21::Pkg211::P211"))
        (kind specialization) (ordinal 0) (authored-target "P12")
        (range (start 13 27) (end 13 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 23) (end 10 27)) (probe (position 10 23))
      (reference
        (source (document "d0") (qualified-name "ImportTest::Pkg2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Pkg1::*")
        (range (start 10 23) (end 10 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 20) (end 4 26)) (probe (position 4 20))
      (reference
        (source (document "d0") (qualified-name "ImportTest::Pkg1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Pkg211::*")
        (range (start 4 20) (end 4 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 20) (end 3 31)) (probe (position 3 20))
      (reference
        (source (document "d0") (qualified-name "ImportTest::Pkg1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Pkg2::Pkg21::*")
        (range (start 3 20) (end 3 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 19) (end 5 31)) (probe (position 5 19))
      (reference
        (source (document "d0") (qualified-name "ImportTest::Pkg1::p11"))
        (kind featureTyping) (ordinal 0) (authored-target "Pkg211::P211")
        (range (start 5 19) (end 5 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 20) (end 2 45)) (probe (position 2 20))
      (reference
        (source (document "d0") (qualified-name "ImportTest::Pkg1::P211"))
        (kind membershipImport) (ordinal 0) (authored-target "Pkg2::Pkg21::Pkg211::P211")
        (range (start 2 20) (end 2 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
