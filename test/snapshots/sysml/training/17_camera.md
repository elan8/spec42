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
  (document "memory://snapshot/17_camera.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 8 2) (end 9 2))
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
        (source "parser")
        (range (start 12 3) (end 14 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 16 2) (end 16 50))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 19 3) (end 21 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9d3da3c4c2b2ce9785a4a68af1284a0f6a44e27c4e7d3fc7108f8423d0a23caa") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Action Decomposition") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::FocusingSubsystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::ImagingSubsystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera"))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::photos"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Action Decomposition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")))))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::photos"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (target (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_camera.md") (range (start 1 16) (end 1 41)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Action Decomposition")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 7 15) (end 7 21)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")))))
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 9 16) (end 9 23)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::photos"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status unresolved)))
  )
)
~~~
