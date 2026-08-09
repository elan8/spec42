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
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (name "xrsl") (declared-name "xrsl") (effective (featuring-type (node (document "d0") (qualified-name "PictureTaking::Focus")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "PictureTaking::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (name "xsf") (declared-name "xsf") (effective (featuring-type (node (document "d0") (qualified-name "PictureTaking::Shoot")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (name "takePicture") (declared-name "takePicture") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (name "focus") (declared-name "focus") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))))
            (element (kind "action") (id (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (name "shoot") (declared-name "shoot") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))))
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (to (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (to (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (to (node (document "d0") (qualified-name "PictureTaking::Exposure"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (to (node (document "d0") (qualified-name "PictureTaking::Exposure"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (to (node (document "d0") (qualified-name "PictureTaking::Focus"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (to (node (document "d0") (qualified-name "PictureTaking::Shoot"))))
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
  (document "sysml/examples/picture_taking.md"
    (diagnostics
    )
  )
)
~~~
