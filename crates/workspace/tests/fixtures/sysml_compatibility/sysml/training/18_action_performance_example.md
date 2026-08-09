# META
~~~ini
description=SysML Training 18 (Action Performance): Action Performance Example
type=file
~~~
# SOURCE
~~~sysml
package 'Action Performance Example' {
	private import 'Action Decomposition'::*;
	
	part def Camera;
	part def AutoFocus;
	part def Imager;
	
	part camera : Camera {
		
		perform action takePhoto[*] ordered 
			references takePicture;
		
		part f : AutoFocus {
			perform takePhoto.focus;			
		}
		
		part i : Imager {
			perform takePhoto.shoot;
		}		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,KwAction,Ident,OpenSquare,Star,CloseSquare,KwOrdered,
KwReferences,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Performance Example''
    (import_decl private ''Action Decomposition'::*')
    (part_def 'Camera')
    (part_def 'AutoFocus')
    (part_def 'Imager')
    (part_usage 'camera' : 'Camera'
      (perform_action 'takePhoto' references 'takePicture' multiplicity ordered)
      (part_usage 'f' : 'AutoFocus'
        (perform_action :>> 'takePhoto.focus'))
      (part_usage 'i' : 'Imager'
        (perform_action :>> 'takePhoto.shoot')))))
~~~
# FORMAT
~~~sysml
package 'Action Performance Example' {
    private import 'Action Decomposition'::*;

    part def Camera;
    part def AutoFocus;
    part def Imager;

    part camera : Camera {

        perform action takePhoto[*] ordered
        references takePicture;

        part f : AutoFocus {
            perform takePhoto.focus;
        }

        part i : Imager {
            perform takePhoto.shoot;
        }
    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'takePicture'
semantic.unresolved_name 'takePhoto::focus'
semantic.unresolved_name 'takePhoto::shoot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'takePicture'
semantic.unresolved_name 'takePhoto::focus'
semantic.unresolved_name 'takePhoto::shoot'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Action Performance Example"))) (name "Action Performance Example") (declared-name "Action Performance Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Action Performance Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Action Performance Example::AutoFocus"))) (name "AutoFocus") (declared-name "AutoFocus") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Action Performance Example::Camera"))) (name "Camera") (declared-name "Camera") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Action Performance Example::Imager"))) (name "Imager") (declared-name "Imager") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Action Performance Example::camera"))) (name "camera") (declared-name "camera") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (name "f") (declared-name "f") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Action Performance Example::Camera"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Action Performance Example::camera::f::takePhoto.focus"))) (name "takePhoto.focus") (declared-name "takePhoto.focus") (effective (featuring-type (node (document "d0") (qualified-name "Action Performance Example::AutoFocus")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (name "i") (declared-name "i") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Action Performance Example::Camera"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Action Performance Example::camera::i::takePhoto.shoot"))) (name "takePhoto.shoot") (declared-name "takePhoto.shoot") (effective (featuring-type (node (document "d0") (qualified-name "Action Performance Example::Imager")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Performance Example::camera"))) (to (node (document "d0") (qualified-name "Action Performance Example::Camera"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (to (node (document "d0") (qualified-name "Action Performance Example::AutoFocus"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (to (node (document "d0") (qualified-name "Action Performance Example::Imager"))) (provenance authored))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "Action Performance Example::camera::f") (target-qualified "Action Performance Example::camera::f::takePhoto::focus"))
    (perform (status pending) (document "d0") (source-qualified "Action Performance Example::camera::i") (target-qualified "Action Performance Example::camera::i::takePhoto::shoot"))
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::AutoFocus"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::Camera"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::Imager"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::camera"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::camera::f::takePhoto.focus"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Performance Example::camera::i::takePhoto.shoot"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/18_action_performance_example.md"
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
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 9 2) (end 9 71))
      )
    )
  )
)
~~~
