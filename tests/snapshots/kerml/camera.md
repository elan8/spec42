# META
~~~ini
description=KerML Behavior: Camera
type=file
~~~
# SOURCE
~~~kerml
class Camera {
	private import ScalarValues::*;
	
	portion focusedState: Camera subsets timeSlices;
	portion shotState: Camera subsets timeSlices;
	
	succession focusedState then shotState;
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
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 38) (end 3 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 35) (end 4 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:949760c1f15c1ac357d2c6a1a56ad4841145fcab98e62ffffe81cfe2104554d8") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "focusedState")) (succession (reference "shotState")))))
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera")) (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera")) (subsetting (reference "timeSlices")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "focusedState")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState")))))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "shotState")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState")))))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind featureTyping) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind featureTyping) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))
      (subtype (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState")) (scopes any))
      (subtype (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))
    )
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState")))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))
      (type (node (document "memory://snapshot/camera.md") (qualified-name "Camera")) (provenance authored))
      (effective-type (node (document "memory://snapshot/camera.md") (qualified-name "Camera")) (source direct))
      (supertype (node (document "memory://snapshot/camera.md") (qualified-name "Camera")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState")))
      (featured-by (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))
      (type (node (document "memory://snapshot/camera.md") (qualified-name "Camera")) (provenance authored))
      (effective-type (node (document "memory://snapshot/camera.md") (qualified-name "Camera")) (source direct))
      (supertype (node (document "memory://snapshot/camera.md") (qualified-name "Camera")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/camera.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 6 12) (end 6 24)) (probe (position 6 12))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "focusedState")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState")))))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 6 30) (end 6 39)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/camera.md") (path (named (kind class-def) (name "Camera")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "shotState")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState")))))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 3 23) (end 3 29)) (probe (position 3 23))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind featureTyping) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 3 38) (end 3 48)) (probe (position 3 38))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 4 20) (end 4 26)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind featureTyping) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera")))))
    )
  )
  (query (document "memory://snapshot/camera.md") (range (start 4 35) (end 4 45)) (probe (position 4 35))
    (reference (id (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
)
~~~
