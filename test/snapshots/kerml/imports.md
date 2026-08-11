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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "e6e2b2da1cb02327a8a66f7174085f3d2a758b0350083ae40cda8470f67dd2f1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Imports"))) (kind "package") (name "Imports") (declared-name "Imports") (range (start (line 0) (character 0)) (end (line 0) (character 480))))
    (element (id (node (document "d0") (qualified-name "Imports::P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 2) (character 1)) (end (line 2) (character 48))) (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::P::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 3) (character 2)) (end (line 3) (character 10))) (parent (node (document "d0") (qualified-name "Imports::P"))))
    (element (id (node (document "d0") (qualified-name "Imports::P::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 4) (character 2)) (end (line 4) (character 10))) (parent (node (document "d0") (qualified-name "Imports::P"))))
    (element (id (node (document "d0") (qualified-name "Imports::P::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 5) (character 2)) (end (line 5) (character 10))) (parent (node (document "d0") (qualified-name "Imports::P"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q"))) (kind "package") (name "Q") (declared-name "Q") (range (start (line 8) (character 1)) (end (line 8) (character 171))) (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 9) (character 2)) (end (line 9) (character 10))) (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::D"))) (kind "classifier decl") (name "D") (declared-name "D") (range (start (line 10) (character 2)) (end (line 10) (character 27))) (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q1"))) (kind "package") (name "Q1") (declared-name "Q1") (range (start (line 13) (character 2)) (end (line 13) (character 85))) (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q1::D"))) (kind "classifier decl") (name "D") (declared-name "D") (range (start (line 14) (character 3)) (end (line 14) (character 11))) (parent (node (document "d0") (qualified-name "Imports::Q::Q1"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q1::E"))) (kind "classifier decl") (name "E") (declared-name "E") (range (start (line 15) (character 3)) (end (line 15) (character 11))) (parent (node (document "d0") (qualified-name "Imports::Q::Q1"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q2"))) (kind "package") (name "Q2") (declared-name "Q2") (range (start (line 20) (character 2)) (end (line 20) (character 30))) (parent (node (document "d0") (qualified-name "Imports::Q"))))
    (element (id (node (document "d0") (qualified-name "Imports::Q::Q2::F"))) (kind "classifier decl") (name "F") (declared-name "F") (range (start (line 21) (character 3)) (end (line 21) (character 11))) (parent (node (document "d0") (qualified-name "Imports::Q::Q2"))))
    (element (id (node (document "d0") (qualified-name "Imports::R"))) (kind "package") (name "R") (declared-name "R") (range (start (line 25) (character 1)) (end (line 25) (character 37))) (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::R::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 26) (character 2)) (end (line 26) (character 21))) (parent (node (document "d0") (qualified-name "Imports::R"))) (authored (membership (kind Import) (visibility "public") (import (reference "Q::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 26) (character 16)) (end (line 26) (character 17))))))
    (element (id (node (document "d0") (qualified-name "Imports::S"))) (kind "package") (name "S") (declared-name "S") (range (start (line 30) (character 1)) (end (line 30) (character 111))) (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::S1"))) (kind "package") (name "S1") (declared-name "S1") (range (start (line 39) (character 1)) (end (line 39) (character 79))) (parent (node (document "d0") (qualified-name "Imports"))))
    (element (id (node (document "d0") (qualified-name "Imports::S1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 40) (character 2)) (end (line 40) (character 21))) (parent (node (document "d0") (qualified-name "Imports::S1"))) (authored (membership (kind Import) (visibility "public") (import (reference "P::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 40) (character 16)) (end (line 40) (character 17))))))
    (element (id (node (document "d0") (qualified-name "Imports::S1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 41) (character 2)) (end (line 41) (character 21))) (parent (node (document "d0") (qualified-name "Imports::S1"))) (authored (membership (kind Import) (visibility "public") (import (reference "R::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 41) (character 16)) (end (line 41) (character 17))))))
    (element (id (node (document "d0") (qualified-name "Imports::S1::X"))) (kind "classifier decl") (name "X") (declared-name "X") (range (start (line 43) (character 2)) (end (line 43) (character 15))) (parent (node (document "d0") (qualified-name "Imports::S1"))))
    (element (id (node (document "d0") (qualified-name "Imports::S::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 31) (character 2)) (end (line 31) (character 21))) (parent (node (document "d0") (qualified-name "Imports::S"))) (authored (membership (kind Import) (visibility "public") (import (reference "P::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 31) (character 16)) (end (line 31) (character 17))))))
    (element (id (node (document "d0") (qualified-name "Imports::S::Q"))) (kind "import") (name "Q") (declared-name "Q") (range (start (line 32) (character 2)) (end (line 32) (character 22))) (parent (node (document "d0") (qualified-name "Imports::S"))) (authored (membership (kind Import) (visibility "public") (import (reference "Q") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 32) (character 16)) (end (line 32) (character 17))))))
    (element (id (node (document "d0") (qualified-name "Imports::S::X"))) (kind "classifier decl") (name "X") (declared-name "X") (range (start (line 34) (character 2)) (end (line 34) (character 15))) (parent (node (document "d0") (qualified-name "Imports::S"))))
    (element (id (node (document "d0") (qualified-name "Imports::S::Y"))) (kind "classifier decl") (name "Y") (declared-name "Y") (range (start (line 35) (character 2)) (end (line 35) (character 15))) (parent (node (document "d0") (qualified-name "Imports::S"))))
    (element (id (node (document "d0") (qualified-name "Imports::S::Z"))) (kind "classifier decl") (name "Z") (declared-name "Z") (range (start (line 36) (character 2)) (end (line 36) (character 15))) (parent (node (document "d0") (qualified-name "Imports::S"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Imports::R::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Q::*") (range (start (line 26) (character 16)) (end (line 26) (character 17))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P::*") (range (start (line 40) (character 16)) (end (line 40) (character 17))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "R::*") (range (start (line 41) (character 16)) (end (line 41) (character 17))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "P::*") (range (start (line 31) (character 16)) (end (line 31) (character 17))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Imports::S::Q"))) (kind membershipImport) (ordinal 0)) (authored-target "Q") (range (start (line 32) (character 16)) (end (line 32) (character 17))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
