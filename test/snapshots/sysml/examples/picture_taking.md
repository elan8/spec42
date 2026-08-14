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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3330ae8b3a68d2bed4e6ba537d9454d61aecf30edd80f5a25040ff04ffa1de2a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Exposure") (direction out))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Exposure") (direction in))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "focus::xrsl")) (memberAccessOperand (reference "shoot::xsf")) (flowPayloadType (reference "Exposure"))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "focus::xrsl")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "shoot::xsf")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf")))))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind flowPayloadType) (ordinal 0))
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
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/picture_taking.md") (range (start 4 28) (end 4 36)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf"))) (kind featureTyping) (ordinal 0) (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 8 24) (end 8 34)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "focus::xrsl")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus::xrsl")))))
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 8 38) (end 8 47)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "shoot::xsf")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot::xsf")))))
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 8 10) (end 8 18)) (probe (position 8 10))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (path (named (kind package) (name "PictureTaking")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))) (kind flowPayloadType) (ordinal 0) (authored-target "Exposure")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Exposure")))))
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Focus")))))
  )
  (query (document "memory://snapshot/picture_taking.md") (range (start 9 16) (end 9 21)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/picture_taking.md") (qualified-name "PictureTaking::Shoot")))))
  )
)
~~~
