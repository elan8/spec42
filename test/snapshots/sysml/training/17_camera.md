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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_camera.md"
    (diagnostics
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
        (range (start 8 2) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 16) (end 9 23))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 12 3) (end 12 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 7) (end 16 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 30) (end 16 49))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6081ff884a1c2781bb2d6875ab62214b65cfc5910d64acb498b2a0d70a00e984") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Camera"))) (kind "package") (name "Camera") (declared-name "Camera") (range (start (line 0) (character 0)) (end (line 0) (character 486))))
    (element (id (node (document "d0") (qualified-name "Camera::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 42))) (parent (node (document "d0") (qualified-name "Camera"))) (authored (membership (kind Import) (visibility "private") (import (reference "Action Decomposition::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Camera::Camera"))) (kind "part def") (name "Camera") (declared-name "Camera") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "Camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::FocusingSubsystem"))) (kind "part def") (name "FocusingSubsystem") (declared-name "FocusingSubsystem") (range (start (line 4) (character 1)) (end (line 4) (character 28))) (parent (node (document "d0") (qualified-name "Camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::ImagingSubsystem"))) (kind "part def") (name "ImagingSubsystem") (declared-name "ImagingSubsystem") (range (start (line 5) (character 1)) (end (line 5) (character 27))) (parent (node (document "d0") (qualified-name "Camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::camera"))) (kind "part") (name "camera") (declared-name "camera") (range (start (line 7) (character 1)) (end (line 7) (character 345))) (parent (node (document "d0") (qualified-name "Camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "Camera") (range (start (line 7) (character 15)) (end (line 7) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Camera::camera::autoFocus"))) (kind "part") (name "autoFocus") (declared-name "autoFocus") (range (start (line 11) (character 2)) (end (line 11) (character 105))) (parent (node (document "d0") (qualified-name "Camera::camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::camera::imager"))) (kind "part") (name "imager") (declared-name "imager") (range (start (line 18) (character 2)) (end (line 18) (character 93))) (parent (node (document "d0") (qualified-name "Camera::camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::camera::photos"))) (kind "part") (name "photos") (declared-name "photos") (range (start (line 9) (character 2)) (end (line 9) (character 27))) (parent (node (document "d0") (qualified-name "Camera::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture") (range (start (line 9) (character 16)) (end (line 9) (character 23)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Camera::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Action Decomposition::*") (range (start (line 1) (character 16)) (end (line 1) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0)) (authored-target "Camera") (range (start (line 7) (character 15)) (end (line 7) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Camera::Camera")))))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera"))) (kind flowSource) (ordinal 0)) (authored-target "autoFocus::realImage") (range (start (line 16) (character 7)) (end (line 16) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera"))) (kind flowTarget) (ordinal 0)) (authored-target "imager::focusedImage") (range (start (line 16) (character 30)) (end (line 16) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera::photos"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range (start (line 9) (character 16)) (end (line 9) (character 23))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Camera::camera"))) (target (node (document "d0") (qualified-name "Camera::Camera"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
