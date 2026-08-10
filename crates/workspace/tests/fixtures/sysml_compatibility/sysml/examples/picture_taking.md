# META
~~~ini
description=SysML Example (Camera): PictureTaking
type=file
~~~
# SOURCE
~~~sysml
package PictureTaking {
	part def Exposure;
	
	action def Focus { out xrsl: Exposure; }
	action def Shoot { in xsf: Exposure; }	
		
	action takePicture {		
		action focus: Focus[1];
		flow of Exposure from focus.xrsl to shoot.xsf;
		action shoot: Shoot[1];
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFlow,KwOf,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'PictureTaking'
    (part_def 'Exposure')
    (action_def 'Focus'
      (default_ref_usage out 'xrsl' : 'Exposure'))
    (action_def 'Shoot'
      (default_ref_usage in 'xsf' : 'Exposure'))
    (action_usage 'takePicture'
      (action_usage 'focus' : 'Focus' multiplicity)
      (flow_usage 'of')
      (action_usage 'shoot' : 'Shoot' multiplicity))))
~~~
# FORMAT
~~~sysml
package PictureTaking {
    part def Exposure;

    action def Focus { out xrsl: Exposure; }
    action def Shoot { in xsf: Exposure; }

    action takePicture {
        action focus: Focus[1];
        flow of Exposure from focus.xrsl to shoot.xsf;
        action shoot: Shoot[1];
    }
}

~~~
# EXPECTED
~~~
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "PictureTaking"))) (name "PictureTaking") (declared-name "PictureTaking")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "PictureTaking::Exposure"))) (name "Exposure") (declared-name "Exposure") (declared))
        (element (kind "action def") (id (node (document "d0") (qualified-name "PictureTaking::Focus"))) (name "Focus") (declared-name "Focus")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (name "xrsl") (declared-name "xrsl") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "PictureTaking::Focus")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "PictureTaking::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (name "xsf") (declared-name "xsf") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "PictureTaking::Shoot")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (name "takePicture") (declared-name "takePicture") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (name "focus") (declared-name "focus") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
            (element (kind "action") (id (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (name "shoot") (declared-name "shoot") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (to (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (to (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (to (node (document "d0") (qualified-name "PictureTaking::Exposure"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (to (node (document "d0") (qualified-name "PictureTaking::Exposure"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (to (node (document "d0") (qualified-name "PictureTaking::Focus"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (to (node (document "d0") (qualified-name "PictureTaking::Shoot"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "PictureTaking::Exposure"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "PictureTaking::Focus"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "PictureTaking::Shoot"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/picture_taking.md"
    (diagnostics
    )
  )
)
~~~
