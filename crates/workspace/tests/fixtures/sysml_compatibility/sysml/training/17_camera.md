# META
~~~ini
description=SysML Training 17 (Control): Camera
type=file
~~~
# SOURCE
~~~sysml
package Camera {
	private import 'Action Decomposition'::*;
	
	part def Camera;
	part def FocusingSubsystem;
	part def ImagingSubsystem;
	
	part camera : Camera {
		ref item scene : Scene;
		part photos : Picture[*];
				
		part autoFocus {
			in ref item scene : Scene = camera::scene;		
			out ref item realImage : Image;
		}
		
		flow autoFocus.realImage to imager.focusedImage;
		
		part imager {
			in item focusedImage : Image;		
			out item photo : Picture :> photos;
		}
		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwRef,KwItem,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenCurly,
KwIn,KwRef,KwItem,Ident,Colon,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwOut,KwRef,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Camera'
    (import_decl private ''Action Decomposition'::*')
    (part_def 'Camera')
    (part_def 'FocusingSubsystem')
    (part_def 'ImagingSubsystem')
    (part_usage 'camera' : 'Camera'
      (item_usage ref 'scene' : 'Scene')
      (part_usage 'photos' : 'Picture' multiplicity)
      (part_usage 'autoFocus'
        (item_usage in ref 'scene' : 'Scene' value)
        (item_usage out ref 'realImage' : 'Image'))
      (flow_usage 'autoFocus')
      (part_usage 'imager'
        (item_usage in 'focusedImage' : 'Image')
        (item_usage out 'photo' : 'Picture' :> 'photos')))))
~~~
# FORMAT
~~~sysml
package Camera {
    private import 'Action Decomposition'::*;

    part def Camera;
    part def FocusingSubsystem;
    part def ImagingSubsystem;

    part camera : Camera {
        ref item scene : Scene;
        part photos : Picture[*];

        part autoFocus {
            in ref item scene : Scene = camera::scene;
            out ref item realImage : Image;
        }

        flow autoFocus.realImage to imager.focusedImage;

        part imager {
            in item focusedImage : Image;
            out item photo : Picture :> photos;
        }

    }
}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'autoFocus'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Picture'
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Picture'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'autoFocus'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Picture'
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Picture'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Camera"))) (name "Camera") (declared-name "Camera")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Camera::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Camera::Camera"))) (name "Camera") (declared-name "Camera") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Camera::FocusingSubsystem"))) (name "FocusingSubsystem") (declared-name "FocusingSubsystem") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Camera::ImagingSubsystem"))) (name "ImagingSubsystem") (declared-name "ImagingSubsystem") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Camera::camera"))) (name "camera") (declared-name "camera") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Camera::camera::autoFocus"))) (name "autoFocus") (declared-name "autoFocus") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Camera::Camera")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Camera::camera::imager"))) (name "imager") (declared-name "imager") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Camera::Camera")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Camera::camera::photos"))) (name "photos") (declared-name "photos") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Camera::Camera")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Camera::camera"))) (to (node (document "d0") (qualified-name "Camera::Camera"))))
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
  (document "sysml/training/17_camera.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 42))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 8 2) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 2) (end 9 27))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 12 3) (end 12 85))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 19 3) (end 19 76))
      )
    )
  )
)
~~~
