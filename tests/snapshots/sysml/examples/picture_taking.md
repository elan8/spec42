# META
~~~ini
description=SysML Example (Camera): PictureTaking
type=file
~~~
# SOURCE
~~~sysml
package PictureTaking {
	part def Exposure;
	
	action def Focus { out xrsl: Exposure; }
	action def Shoot { in xsf: Exposure; }	
		
	action takePicture {		
		action focus: Focus[1];
		flow of Exposure from focus.xrsl to shoot.xsf;
		action shoot: Shoot[1];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/picture_taking.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 24) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 38) (end 8 47))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3330ae8b3a68d2bed4e6ba537d9454d61aecf30edd80f5a25040ff04ffa1de2a") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Exposure") (direction out)))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Exposure") (direction in)))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "focus::xrsl")) (flowTarget (reference "shoot::xsf")) (flowPayloadType (reference "Exposure")))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus")))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "focus::xrsl")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "shoot::xsf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0))
      (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))
      (subtype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl")) (scopes any))
      (subtype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")))
      (subtype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl")))
      (featured-by (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")))
      (type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")) (source direct))
      (supertype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")))
      (subtype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf")))
      (featured-by (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")))
      (type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")) (source direct))
      (supertype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture")))
    )
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus")))
      (featured-by (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture")))
      (type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")) (source direct))
      (supertype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot")))
      (featured-by (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture")))
      (type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")) (source direct))
      (supertype (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/picture_taking.md") (range (start 3 30) (end 3 38)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (kind featureTyping) (ordinal 0) (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    )
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 4 28) (end 4 36)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (kind featureTyping) (ordinal 0) (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    )
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 8 24) (end 8 34)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "focus::xrsl")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 8 38) (end 8 47)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "shoot::xsf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 8 10) (end 8 18)) (probe (position 8 10))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0) (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    )
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")))))
    )
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 9 16) (end 9 21)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")))))
    )
  )
)
~~~
