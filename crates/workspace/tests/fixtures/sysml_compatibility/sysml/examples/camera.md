# META
~~~ini
description=SysML Example (Camera): Camera
type=file
~~~
# SOURCE
~~~sysml
part def Camera {
	private import PictureTaking::*;
	
	perform action takePicture[*] :> PictureTaking::takePicture;
	
	part focusingSubsystem {
		perform takePicture.focus;
	}
	
	part imagingSubsystem {
		perform takePicture.shoot;
	}
	
}
~~~
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPerform,KwAction,Ident,OpenSquare,Star,CloseSquare,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Camera'
    (import_decl private 'PictureTaking::*')
    (perform_action 'takePicture' :> 'PictureTaking::takePicture' multiplicity)
    (part_usage 'focusingSubsystem'
      (perform_action :>> 'takePicture.focus'))
    (part_usage 'imagingSubsystem'
      (perform_action :>> 'takePicture.shoot'))))
~~~
# FORMAT
~~~sysml
part def Camera {
    private import PictureTaking::*;

    perform action takePicture[*] :> PictureTaking::takePicture;

    part focusingSubsystem {
        perform takePicture.focus;
    }

    part imagingSubsystem {
        perform takePicture.shoot;
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'PictureTaking::takePicture'
semantic.unresolved_name 'takePicture::focus'
semantic.unresolved_name 'takePicture::shoot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'PictureTaking::takePicture'
semantic.unresolved_name 'takePicture::focus'
semantic.unresolved_name 'takePicture::shoot'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "part def") (id (node (document "d0") (qualified-name "Camera"))) (name "Camera") (declared-name "Camera") (declared)
      (contains
        (element (kind "part") (id (node (document "d0") (qualified-name "Camera::focusingSubsystem"))) (name "focusingSubsystem") (declared-name "focusingSubsystem") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Camera"))))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Camera::focusingSubsystem::takePicture.focus"))) (name "takePicture.focus") (declared-name "takePicture.focus") (effective (featuring-type (node (document "d0") (qualified-name "Camera")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Camera::imagingSubsystem"))) (name "imagingSubsystem") (declared-name "imagingSubsystem") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Camera"))))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Camera::imagingSubsystem::takePicture.shoot"))) (name "takePicture.shoot") (declared-name "takePicture.shoot") (effective (featuring-type (node (document "d0") (qualified-name "Camera")))))
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "Camera::focusingSubsystem") (target-qualified "Camera::focusingSubsystem::takePicture::focus"))
    (perform (status pending) (document "d0") (source-qualified "Camera::imagingSubsystem") (target-qualified "Camera::imagingSubsystem::takePicture::shoot"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/camera.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 3 1) (end 3 65))
      )
    )
  )
)
~~~
