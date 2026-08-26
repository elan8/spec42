# META
~~~ini
description=KerML Behavior: TakePicture
type=file
~~~
# SOURCE
~~~kerml
behavior TakePicture {
	private import Camera;
	
	feature camera: Camera[1] subsets involvedObjects;
	
	class Exposure;
	
	behavior Focus { out xrsl: Exposure; }
	behavior Shoot { in xsf: Exposure; }
	
	step step1: Focus[1];	
	step step2: Shoot[1];
	
	succession flow exposure[1] of Exposure from step1.xrsl to step2.xsf;

	succession step1 then camera.focusedState;
	succession step2 then camera.shotState;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/take_picture.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 17) (end 3 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 35) (end 3 50))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 13 1) (end 15 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 23) (end 15 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 23) (end 16 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3858e8d89aa17eedefc4517dd85160687281f32d72516dcb42ae47bfe6a63a17") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Camera") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "step1")) (succession (reference "camera::focusedState")))))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "step2")) (succession (reference "camera::shotState")))))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Exposure") (direction out)))))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Exposure") (direction in)))))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::camera"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera")) (subsetting (reference "involvedObjects")))))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus")))))
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Camera")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "step1")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1")))))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "step2")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2")))))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "camera::focusedState")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "camera::shotState")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")))))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")))))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::camera"))) (kind featureTyping) (ordinal 0))
      (authored-target "Camera")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::camera"))) (kind subsetting) (ordinal 0))
      (authored-target "involvedObjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus")))))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::camera"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2"))) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture")))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture")))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")))
      (subtype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl")) (scopes any))
      (subtype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus")))
      (subtype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl")))
      (featured-by (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus")))
      (type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")) (source direct))
      (supertype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot")))
      (subtype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf")))
      (featured-by (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot")))
      (type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")) (source direct))
      (supertype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::camera")))
      (featured-by (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture")))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1")))
      (featured-by (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture")))
      (type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus")) (source direct))
      (supertype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2")))
      (featured-by (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture")))
      (type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot")) (source direct))
      (supertype (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/take_picture.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Camera")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 15 12) (end 15 17)) (probe (position 15 12))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "step1")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1")))))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 16 12) (end 16 17)) (probe (position 16 12))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "step2")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2")))))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 15 23) (end 15 42)) (probe (position 15 23))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "camera::focusedState")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 16 23) (end 16 39)) (probe (position 16 23))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (path (named (kind kerml-behavior) (name "TakePicture")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "camera::shotState")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 7 28) (end 7 36)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus::xrsl"))) (kind featureTyping) (ordinal 0) (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")))))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 8 26) (end 8 34)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot::xsf"))) (kind featureTyping) (ordinal 0) (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Exposure")))))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 3 17) (end 3 23)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::camera"))) (kind featureTyping) (ordinal 0) (authored-target "Camera")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 3 35) (end 3 50)) (probe (position 3 35))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::camera"))) (kind subsetting) (ordinal 0) (authored-target "involvedObjects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 10 13) (end 10 18)) (probe (position 10 13))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step1"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Focus")))))
    )
  )
  (query (document "memory://snapshot/take_picture.md") (range (start 11 13) (end 11 18)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::step2"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/take_picture.md") (qualified-name "TakePicture::Shoot")))))
    )
  )
)
~~~
