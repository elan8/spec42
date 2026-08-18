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
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_parser_construct")
        (source "semantic")
        (range (start 1 1) (end 1 32))
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
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 6 1) (end 7 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:949760c1f15c1ac357d2c6a1a56ad4841145fcab98e62ffffe81cfe2104554d8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera")) (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera")) (subsetting (reference "timeSlices")))))
  )
  (references
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
    (relationship (kind typing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::focusedState"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (target (node (document "memory://snapshot/camera.md") (qualified-name "Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/camera.md") (qualified-name "Camera::shotState"))) (kind featureTyping) (ordinal 0)))
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
