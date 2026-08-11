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
  (document "picture_taking.md"
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d8729fb09d38ed5ef917192bc888ff4f6d24446c820cd13993065900ea79b7a9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "PictureTaking"))) (kind "package") (name "PictureTaking") (declared-name "PictureTaking") (range (start (line 0) (character 0)) (end (line 0) (character 261))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::Exposure"))) (kind "part def") (name "Exposure") (declared-name "Exposure") (range (start (line 1) (character 1)) (end (line 1) (character 19))) (parent (node (document "d0") (qualified-name "PictureTaking"))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 3) (character 1)) (end (line 3) (character 41))) (parent (node (document "d0") (qualified-name "PictureTaking"))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (kind "in out parameter") (name "xrsl") (declared-name "xrsl") (range (start (line 3) (character 20)) (end (line 3) (character 39))) (parent (node (document "d0") (qualified-name "PictureTaking::Focus"))) (authored (relationships (typing (reference "Exposure") (range none)))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 4) (character 1)) (end (line 4) (character 39))) (parent (node (document "d0") (qualified-name "PictureTaking"))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (kind "in out parameter") (name "xsf") (declared-name "xsf") (range (start (line 4) (character 20)) (end (line 4) (character 37))) (parent (node (document "d0") (qualified-name "PictureTaking::Shoot"))) (authored (relationships (typing (reference "Exposure") (range none)))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (range (start (line 6) (character 1)) (end (line 6) (character 127))) (parent (node (document "d0") (qualified-name "PictureTaking"))) (authored (membership (kind Feature)) (relationships (perform (reference "PictureTaking::takePicture::focus") (range none)) (perform (reference "PictureTaking::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 7) (character 2)) (end (line 7) (character 25))) (parent (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 9) (character 2)) (end (line 9) (character 25))) (parent (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (kind featureTyping) (ordinal 0)) (authored-target "Exposure") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PictureTaking::Exposure")))))
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (kind featureTyping) (ordinal 0)) (authored-target "Exposure") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PictureTaking::Exposure")))))
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::xrsl") (range (start (line 8) (character 24)) (end (line 8) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::xsf") (range (start (line 8) (character 38)) (end (line 8) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "PictureTaking::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PictureTaking::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "PictureTaking::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PictureTaking::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PictureTaking::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (target (node (document "d0") (qualified-name "PictureTaking::Exposure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PictureTaking::Focus::xrsl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (target (node (document "d0") (qualified-name "PictureTaking::Exposure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PictureTaking::Shoot::xsf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (target (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (target (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PictureTaking::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (target (node (document "d0") (qualified-name "PictureTaking::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PictureTaking::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (target (node (document "d0") (qualified-name "PictureTaking::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PictureTaking::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
