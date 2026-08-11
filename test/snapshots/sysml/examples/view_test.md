# META
~~~ini
description=SysML Example (Simple Tests): ViewTest
type=file
~~~
# SOURCE
~~~sysml
package ViewTest {
	package P {
		public part p1;
		private part p2;
	}
	
	part def S;
	
	concern def C {
	    subject;
		stakeholder s : S;
	}
	
	concern c : C {
	    subject;
		stakeholder s1;
	}
	
	viewpoint def VP {
		frame c;
	}
	
	rendering def R;
	
	rendering r : R;
	
	view def V {
		viewpoint vp: VP {
			frame concern c1;
			concern c2;
		}
		render rendering r1: R[0..1]; 
		
		view v: V[0..*] {
			expose P::*;
			render r;
			
			rendering r2;
			
			alias vp1 for p1;
			// Note: "expose" imports all.
			alias vp2 for p2;
		}
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_view_def_body_element")
        (source "sysml")
        (range (start 31 2) (end 31 38))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "66ec1604a6986754b1782da039f5a86ad738e4656070aa7f5550d928f4fb5520") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ViewTest"))) (kind "package") (name "ViewTest") (declared-name "ViewTest"))
    (element (id (node (document "d0") (qualified-name "ViewTest::C"))) (kind "concern def") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::C::s"))) (kind "stakeholder") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "ViewTest::C"))) (authored (relationships (typing (reference "S")))))
    (element (id (node (document "d0") (qualified-name "ViewTest::P"))) (kind "package") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::P::p1"))) (kind "part") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "ViewTest::P"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::P::p2"))) (kind "part") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "ViewTest::P"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::R"))) (kind "rendering def") (name "R") (declared-name "R") (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::S"))) (kind "part def") (name "S") (declared-name "S") (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::V"))) (kind "view def") (name "V") (declared-name "V") (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::VP"))) (kind "viewpoint def") (name "VP") (declared-name "VP") (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::VP::c"))) (kind "frame") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "ViewTest::VP"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::c"))) (kind "concern") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "ViewTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "ViewTest::c::_stakeholder_s1"))) (kind "stakeholder") (name "s1") (declared-name "s1") (parent (node (document "d0") (qualified-name "ViewTest::c"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::r"))) (kind "rendering") (name "r") (declared-name "r") (parent (node (document "d0") (qualified-name "ViewTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "R")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ViewTest::C::s"))) (kind featureTyping) (ordinal 0)) (authored-target "S") (outcome (status resolved) (target (node (document "d0") (qualified-name "ViewTest::S")))))
    (reference (id (source (node (document "d0") (qualified-name "ViewTest::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "ViewTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "ViewTest::r"))) (kind featureTyping) (ordinal 0)) (authored-target "R") (outcome (status resolved) (target (node (document "d0") (qualified-name "ViewTest::R")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ViewTest::C::s"))) (target (node (document "d0") (qualified-name "ViewTest::S"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ViewTest::C::s"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ViewTest::c"))) (target (node (document "d0") (qualified-name "ViewTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ViewTest::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ViewTest::r"))) (target (node (document "d0") (qualified-name "ViewTest::R"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ViewTest::r"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
