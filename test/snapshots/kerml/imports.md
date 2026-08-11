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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "imports.md"
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
        (range (start 26 16) (end 26 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 31 16) (end 31 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 32 16) (end 32 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 40 16) (end 40 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 41 16) (end 41 17))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "e6e2b2da1cb02327a8a66f7174085f3d2a758b0350083ae40cda8470f67dd2f1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Imports"))) (kind "package") (name "Imports") (declared-name "Imports"))
    (element (id (node (document "d0") (qualified-name "Imports::P"))) (kind "package") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::P::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Imports::P"))))
    (element (id (node (document "d0") (qualified-name "Imports::P::B"))) (kind "classifier decl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Imports::P"))))
    (element (id (node (document "d0") (qualified-name "Imports::P::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "Imports::P"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q"))) (kind "package") (name "Q") (declared-name "Q") (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::D"))) (kind "classifier decl") (name "D") (declared-name "D") (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q1"))) (kind "package") (name "Q1") (declared-name "Q1") (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q1::D"))) (kind "classifier decl") (name "D") (declared-name "D") (parent (node (document "d0") (qualified-name "Imports::Q::Q1"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q1::E"))) (kind "classifier decl") (name "E") (declared-name "E") (parent (node (document "d0") (qualified-name "Imports::Q::Q1"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q2"))) (kind "package") (name "Q2") (declared-name "Q2") (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q2::F"))) (kind "classifier decl") (name "F") (declared-name "F") (parent (node (document "d0") (qualified-name "Imports::Q::Q2"))))
    (element (id (node (document "d0") (qualified-name "Imports::R"))) (kind "package") (name "R") (declared-name "R") (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::R::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Imports::R"))) (authored (membership (kind Import) (visibility "public") (import (reference "Q::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Imports::S"))) (kind "package") (name "S") (declared-name "S") (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::S1"))) (kind "package") (name "S1") (declared-name "S1") (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::S1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Imports::S1"))) (authored (membership (kind Import) (visibility "public") (import (reference "P::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Imports::S1::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Imports::S1"))) (authored (membership (kind Import) (visibility "public") (import (reference "R::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Imports::S1::X"))) (kind "classifier decl") (name "X") (declared-name "X") (parent (node (document "d0") (qualified-name "Imports::S1"))))
    (element (id (node (document "d0") (qualified-name "Imports::S::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Imports::S"))) (authored (membership (kind Import) (visibility "public") (import (reference "P::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Imports::S::Q"))) (kind "import") (name "Q") (declared-name "Q") (parent (node (document "d0") (qualified-name "Imports::S"))) (authored (membership (kind Import) (visibility "public") (import (reference "Q") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Imports::S::X"))) (kind "classifier decl") (name "X") (declared-name "X") (parent (node (document "d0") (qualified-name "Imports::S"))))
    (element (id (node (document "d0") (qualified-name "Imports::S::Y"))) (kind "classifier decl") (name "Y") (declared-name "Y") (parent (node (document "d0") (qualified-name "Imports::S"))))
    (element (id (node (document "d0") (qualified-name "Imports::S::Z"))) (kind "classifier decl") (name "Z") (declared-name "Z") (parent (node (document "d0") (qualified-name "Imports::S"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Imports::R::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Q::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "R::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S::Q"))) (kind membershipImport) (ordinal 0)) (authored-target "Q") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive true) (conformance not-checked-unresolved)))
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
    (query (range (start 26 16) (end 26 17)) (probe (position 26 16))
      (reference
        (source (document "d0") (qualified-name "Imports::R::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Q::*")
        (range (start 26 16) (end 26 17))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 16) (end 31 17)) (probe (position 31 16))
      (reference
        (source (document "d0") (qualified-name "Imports::S::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P::*")
        (range (start 31 16) (end 31 17))
        (outcome (status unresolved))
      )
    )
    (query (range (start 32 16) (end 32 17)) (probe (position 32 16))
      (reference
        (source (document "d0") (qualified-name "Imports::S::Q"))
        (kind membershipImport) (ordinal 0) (authored-target "Q")
        (range (start 32 16) (end 32 17))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 16) (end 40 17)) (probe (position 40 16))
      (reference
        (source (document "d0") (qualified-name "Imports::S1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "P::*")
        (range (start 40 16) (end 40 17))
        (outcome (status unresolved))
      )
    )
    (query (range (start 41 16) (end 41 17)) (probe (position 41 16))
      (reference
        (source (document "d0") (qualified-name "Imports::S1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "R::*")
        (range (start 41 16) (end 41 17))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
