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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "fb9810bdb90395330663dcb1b1ec4a030548454aefd0f47a274ee9ef78e5f1b5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Camera"))) (kind "package") (name "Camera") (declared-name "Camera"))
    (element (id (node (document "d0") (qualified-name "Camera::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Camera"))) (authored (membership (kind Import) (visibility "private") (import (reference "Action Decomposition::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Camera::Camera"))) (kind "part def") (name "Camera") (declared-name "Camera") (parent (node (document "d0") (qualified-name "Camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::FocusingSubsystem"))) (kind "part def") (name "FocusingSubsystem") (declared-name "FocusingSubsystem") (parent (node (document "d0") (qualified-name "Camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::ImagingSubsystem"))) (kind "part def") (name "ImagingSubsystem") (declared-name "ImagingSubsystem") (parent (node (document "d0") (qualified-name "Camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::camera"))) (kind "part") (name "camera") (declared-name "camera") (parent (node (document "d0") (qualified-name "Camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "Camera")))))
    (element (id (node (document "d0") (qualified-name "Camera::camera::autoFocus"))) (kind "part") (name "autoFocus") (declared-name "autoFocus") (parent (node (document "d0") (qualified-name "Camera::camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::camera::imager"))) (kind "part") (name "imager") (declared-name "imager") (parent (node (document "d0") (qualified-name "Camera::camera"))))
    (element (id (node (document "d0") (qualified-name "Camera::camera::photos"))) (kind "part") (name "photos") (declared-name "photos") (parent (node (document "d0") (qualified-name "Camera::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Camera::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Action Decomposition::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0)) (authored-target "Camera") (outcome (status resolved) (target (node (document "d0") (qualified-name "Camera::Camera")))))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera"))) (kind flowSource) (ordinal 0)) (authored-target "autoFocus::realImage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera"))) (kind flowTarget) (ordinal 0)) (authored-target "imager::focusedImage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Camera::camera::photos"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Camera::camera"))) (target (node (document "d0") (qualified-name "Camera::Camera"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 15) (end 7 21)) (probe (position 7 15))
      (reference
        (source (document "d0") (qualified-name "Camera::camera"))
        (kind featureTyping) (ordinal 0) (authored-target "Camera")
        (range (start 7 15) (end 7 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Camera::Camera") (range (start 3 1) (end 3 17)))
        )
      )
    )
    (query (range (start 9 16) (end 9 23)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Camera::camera::photos"))
        (kind featureTyping) (ordinal 0) (authored-target "Picture")
        (range (start 9 16) (end 9 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 7) (end 16 26)) (probe (position 16 7))
      (reference
        (source (document "d0") (qualified-name "Camera::camera"))
        (kind flowSource) (ordinal 0) (authored-target "autoFocus::realImage")
        (range (start 16 7) (end 16 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 30) (end 16 49)) (probe (position 16 30))
      (reference
        (source (document "d0") (qualified-name "Camera::camera"))
        (kind flowTarget) (ordinal 0) (authored-target "imager::focusedImage")
        (range (start 16 30) (end 16 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 38)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Camera::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Action Decomposition::*")
        (range (start 1 16) (end 1 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
