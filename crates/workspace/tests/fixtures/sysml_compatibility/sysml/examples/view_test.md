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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwPart,Ident,Semicolon,
KwPrivate,KwPart,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwConcern,KwDef,Ident,OpenCurly,
KwSubject,Semicolon,
KwStakeholder,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConcern,Ident,Colon,Ident,OpenCurly,
KwSubject,Semicolon,
KwStakeholder,Ident,Semicolon,
CloseCurly,
KwViewpoint,KwDef,Ident,OpenCurly,
KwFrame,Ident,Semicolon,
CloseCurly,
KwRendering,KwDef,Ident,Semicolon,
KwRendering,Ident,Colon,Ident,Semicolon,
KwView,KwDef,Ident,OpenCurly,
KwViewpoint,Ident,Colon,Ident,OpenCurly,
KwFrame,KwConcern,Ident,Semicolon,
KwConcern,Ident,Semicolon,
CloseCurly,
KwRender,KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwView,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwExpose,Ident,ColonColon,Star,Semicolon,
KwRender,Ident,Semicolon,
KwRendering,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
LineComment,
KwAlias,Ident,KwFor,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ViewTest'
    (package_def 'P'
      (part_usage public 'p1')
      (part_usage private 'p2'))
    (part_def 'S')
    (concern_def 'C'
      (sysml_decl)
      (sysml_decl 's' : 'S'))
    (sysml_decl 'c' : 'C'
      (sysml_decl)
      (sysml_decl 's1'))
    (viewpoint_def 'VP'
      (sysml_decl 'c'))
    (rendering_def 'R')
    (sysml_decl 'r' : 'R')
    (view_def 'V'
      (sysml_decl 'vp' : 'VP'
        (sysml_decl 'c1')
        (sysml_decl 'c2'))
      (malformed)
      (sysml_decl 'r1' : 'R' multiplicity)
      (sysml_decl 'v' : 'V' multiplicity
        (expose_member)
        (view_rendering)
        (sysml_decl 'r2')
        (alias_member 'vp1' for 'p1')
        (line_comment)
        (alias_member 'vp2' for 'p2')))))
~~~
# FORMAT
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
# EXPECTED
~~~
parse.expected_semicolon_or_body
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ViewTest"))) (name "ViewTest") (declared-name "ViewTest")
      (contains
        (element (kind "concern def") (id (node (document "d0") (qualified-name "ViewTest::C"))) (name "C") (declared-name "C")
          (contains
            (element (kind "stakeholder") (id (node (document "d0") (qualified-name "ViewTest::C::s"))) (name "s") (declared-name "s") (effective (featuring-type (node (document "d0") (qualified-name "ViewTest::C")))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "ViewTest::P"))) (name "P") (declared-name "P")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ViewTest::P::p1"))) (name "p1") (declared-name "p1") (declared (properties (composite true) (reference false) (ordered false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ViewTest::P::p2"))) (name "p2") (declared-name "p2") (declared (properties (composite true) (reference false) (ordered false))))
          )
        )
        (element (kind "rendering def") (id (node (document "d0") (qualified-name "ViewTest::R"))) (name "R") (declared-name "R"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "ViewTest::S"))) (name "S") (declared-name "S") (declared))
        (element (kind "view def") (id (node (document "d0") (qualified-name "ViewTest::V"))) (name "V") (declared-name "V"))
        (element (kind "viewpoint def") (id (node (document "d0") (qualified-name "ViewTest::VP"))) (name "VP") (declared-name "VP")
          (contains
            (element (kind "frame") (id (node (document "d0") (qualified-name "ViewTest::VP::c"))) (name "c") (declared-name "c") (effective (featuring-type (node (document "d0") (qualified-name "ViewTest::VP")))))
          )
        )
        (element (kind "concern") (id (node (document "d0") (qualified-name "ViewTest::c"))) (name "c") (declared-name "c")
          (contains
            (element (kind "stakeholder") (id (node (document "d0") (qualified-name "ViewTest::c::_stakeholder_s1"))) (name "s1") (declared-name "s1") (effective (featuring-type (node (document "d0") (qualified-name "ViewTest::C")))))
          )
        )
        (element (kind "rendering") (id (node (document "d0") (qualified-name "ViewTest::r"))) (name "r") (declared-name "r"))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "ViewTest::C::s"))) (to (node (document "d0") (qualified-name "ViewTest::S"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ViewTest::c"))) (to (node (document "d0") (qualified-name "ViewTest::C"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ViewTest::r"))) (to (node (document "d0") (qualified-name "ViewTest::R"))))
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
  (document "sysml/examples/view_test.md"
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
