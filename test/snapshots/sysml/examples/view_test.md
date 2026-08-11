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
# EXPECTED
~~~
parse.expected_semicolon_or_body
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "66ec1604a6986754b1782da039f5a86ad738e4656070aa7f5550d928f4fb5520") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ViewTest"))) (kind "package") (name "ViewTest") (declared-name "ViewTest") (range (start (line 0) (character 0)) (end (line 0) (character 546))))
    (element (id (node (document "d0") (qualified-name "ViewTest::C"))) (kind "concern def") (name "C") (declared-name "C") (range (start (line 8) (character 1)) (end (line 8) (character 54))) (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::C::s"))) (kind "stakeholder") (name "s") (declared-name "s") (range (start (line 10) (character 2)) (end (line 10) (character 20))) (parent (node (document "d0") (qualified-name "ViewTest::C"))) (authored (relationships (typing (reference "S") (range none)))))
    (element (id (node (document "d0") (qualified-name "ViewTest::P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 1) (character 1)) (end (line 1) (character 52))) (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::P::p1"))) (kind "part") (name "p1") (declared-name "p1") (range (start (line 2) (character 2)) (end (line 2) (character 17))) (parent (node (document "d0") (qualified-name "ViewTest::P"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::P::p2"))) (kind "part") (name "p2") (declared-name "p2") (range (start (line 3) (character 2)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "ViewTest::P"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::R"))) (kind "rendering def") (name "R") (declared-name "R") (range (start (line 22) (character 1)) (end (line 22) (character 17))) (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::S"))) (kind "part def") (name "S") (declared-name "S") (range (start (line 6) (character 1)) (end (line 6) (character 12))) (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::V"))) (kind "view def") (name "V") (declared-name "V") (range (start (line 26) (character 1)) (end (line 26) (character 267))) (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::VP"))) (kind "viewpoint def") (name "VP") (declared-name "VP") (range (start (line 18) (character 1)) (end (line 18) (character 33))) (parent (node (document "d0") (qualified-name "ViewTest"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::VP::c"))) (kind "frame") (name "c") (declared-name "c") (range (start (line 19) (character 2)) (end (line 19) (character 10))) (parent (node (document "d0") (qualified-name "ViewTest::VP"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::c"))) (kind "concern") (name "c") (declared-name "c") (range (start (line 13) (character 1)) (end (line 13) (character 51))) (parent (node (document "d0") (qualified-name "ViewTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C") (range none)))))
    (element (id (node (document "d0") (qualified-name "ViewTest::c::_stakeholder_s1"))) (kind "stakeholder") (name "s1") (declared-name "s1") (range (start (line 15) (character 2)) (end (line 15) (character 17))) (parent (node (document "d0") (qualified-name "ViewTest::c"))))
    (element (id (node (document "d0") (qualified-name "ViewTest::r"))) (kind "rendering") (name "r") (declared-name "r") (range (start (line 24) (character 1)) (end (line 24) (character 17))) (parent (node (document "d0") (qualified-name "ViewTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "R") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ViewTest::C::s"))) (kind featureTyping) (ordinal 0)) (authored-target "S") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ViewTest::S")))))
    (reference (id (source (node (document "d0") (qualified-name "ViewTest::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ViewTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "ViewTest::r"))) (kind featureTyping) (ordinal 0)) (authored-target "R") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ViewTest::R")))))
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
