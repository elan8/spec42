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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/camera.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 34) (end 3 60))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 1) (end 7 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 10) (end 6 27))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 9 1) (end 11 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 10) (end 10 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:34a79bdb4b0822dc0d09cf0a11f9f27d7aeec66e8ea5ea2088896cf56d4c1122") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PictureTaking") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusingSubsystem"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "focusingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "takePicture::focus")))))
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::imagingSubsystem"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "imagingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "takePicture::shoot")))))
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::takePicture"))) (kind perform-action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "PictureTaking::takePicture")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PictureTaking")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "focusingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "takePicture::focus")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "imagingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "takePicture::shoot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::takePicture"))) (kind subsetting) (ordinal 0))
      (authored-target "PictureTaking::takePicture")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusingSubsystem"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "focusingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusingSubsystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::imagingSubsystem"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "imagingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::imagingSubsystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::takePicture"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusingSubsystem")))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))
    )
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "focusingSubsystem")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusingSubsystem")))
    )
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::imagingSubsystem")))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))
    )
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "imagingSubsystem")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera::imagingSubsystem")))
    )
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::takePicture")))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/camera.md") (range (start 1 16) (end 1 32)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "PictureTaking")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 6 10) (end 6 27)) (probe (position 6 10))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "focusingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "takePicture::focus")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 10 10) (end 10 27)) (probe (position 10 10))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind part-def) (name "Camera")) (named (kind part) (name "imagingSubsystem")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "takePicture::shoot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 3 34) (end 3 60)) (probe (position 3 34))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::takePicture"))) (kind subsetting) (ordinal 0) (authored-target "PictureTaking::takePicture")
      (outcome (status unresolved)))
    )
  )
)
~~~
