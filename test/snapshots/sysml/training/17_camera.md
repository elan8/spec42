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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 19) (end 8 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 16) (end 9 23))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 11 2) (end 14 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 23) (end 12 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 28) (end 13 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 16 2) (end 16 50))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 18 2) (end 21 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 26) (end 19 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 20) (end 20 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:9d3da3c4c2b2ce9785a4a68af1284a0f6a44e27c4e7d3fc7108f8423d0a23caa") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (path (named (kind package) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Action Decomposition") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::FocusingSubsystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::ImagingSubsystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera")))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::realImage"))) (kind ref) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::scene"))) (kind ref) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene")))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::focusedImage"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::photo"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture")))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::photos"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture")))))
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::scene"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (path (named (kind package) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Action Decomposition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")))))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::realImage"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::focusedImage"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::photo"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::photos"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (target (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")))
      (subtype (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera")))
      (type (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")) (provenance authored))
      (effective-type (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")) (source direct))
      (supertype (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera")))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::realImage")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus")))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::scene")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus")))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera")))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::focusedImage")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager")))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::photo")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager")))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::photos")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera")))
    )
    (declaration (id (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::scene")))
      (featured-by (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_camera.md") (range (start 1 16) (end 1 41)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (path (named (kind package) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Action Decomposition")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 7 15) (end 7 21)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera"))) (kind featureTyping) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::Camera")))))
    )
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 13 28) (end 13 33)) (probe (position 13 28))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::realImage"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 12 23) (end 12 28)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::autoFocus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 19 26) (end 19 31)) (probe (position 19 26))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::focusedImage"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 20 20) (end 20 27)) (probe (position 20 20))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::imager::photo"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 9 16) (end 9 23)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::photos"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_camera.md") (range (start 8 19) (end 8 24)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/17_camera.md") (qualified-name "Camera::camera::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status unresolved)))
    )
  )
)
~~~
