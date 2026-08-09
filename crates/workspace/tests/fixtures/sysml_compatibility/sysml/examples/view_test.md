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
        viewpoint vp : VP {
            frame c1;
            concern c2;
        }
        render
        rendering r1 : R [0..1];

        view v : V [0..*] {
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
(model
  (namespace
    (package 'ViewTest'
      (package 'P'
        (part_usage 'p1')
        (part_usage 'p2'))
      (part_def 'S')
      (concern_def 'C'
        (subject_membership in)
        (stakeholder_membership in 's' : 'ViewTest::S'[part_def]))
      (concern_usage 'c' : 'ViewTest::C'[concern_def]
        (subject_membership in)
        (stakeholder_membership in 's1'))
      (viewpoint_def 'VP'
        (framed_concern_membership 'c'))
      (rendering_def 'R')
      (rendering_usage 'r' : 'ViewTest::R'[rendering_def])
      (view_def 'V'
        (viewpoint_usage 'vp' : 'ViewTest::VP'[viewpoint_def]
          (framed_concern_membership 'c1')
          (concern_usage composite 'c2'))
        (not_implemented 'malformed')
        (rendering_usage composite 'r1' : 'ViewTest::R'[rendering_def]
          (multiplicity_range [0..1]))
        (view_usage composite 'v' : 'ViewTest::V'[view_def]
          (multiplicity_range [0..*])
          (namespace_expose all -> 'ViewTest::P'[package])
          (view_rendering_membership -> 'ViewTest::r'[rendering_usage])
          (rendering_usage composite 'r2')
          (alias_member 'vp1' -> 'p1'[unresolved])
          (alias_member 'vp2' -> 'p2'[unresolved]))))))
~~~
