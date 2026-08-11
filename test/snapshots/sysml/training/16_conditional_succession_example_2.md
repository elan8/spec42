# META
~~~ini
description=SysML Training 16 (Conditional Succession): Conditional Succession Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Conditional Succession Example-2' {
	part def Scene;
	part def Image {
		isWellFocused: ScalarValues::Boolean;
	}
	part def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
	action def TakePicture { in scene : Scene; out picture : Picture; }
	
	action takePicture : TakePicture {
		in item scene;
		out item picture;
		
		action focus : Focus {
			in item scene = takePicture::scene; 
			out item image;
		}
		
		if focus.image.isWellFocused then shoot;
		
		flow from focus.image to shoot.image;

		action shoot : Shoot {
			in item image; 
			out item picture = takePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "16_conditional_succession_example_2.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 16 3) (end 16 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 20 2) (end 20 48))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 26 3) (end 26 43))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c745f466697de1b101bf5534b38f55260c2b1d5ebddbf3a5da217f99f387a587") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2"))) (kind "package") (name "Conditional Succession Example-2") (declared-name "Conditional Succession Example-2"))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus"))) (authored (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Image"))) (kind "part def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture"))) (kind "part def") (name "Picture") (declared-name "Picture") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene"))) (kind "part def") (name "Scene") (declared-name "Scene") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot"))) (authored (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture"))) (authored (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture"))) (authored (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture")) (perform (reference "Conditional Succession Example-2::takePicture::focus")) (perform (reference "Conditional Succession Example-2::takePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot")))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Conditional Succession Example-2::takePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Conditional Succession Example-2::takePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::image"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::image"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "focus::image") (target "shoot::image")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::scene")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::picture")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 22 12) (end 22 23)) (probe (position 22 12))
      (reference
        (source (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))
        (kind flowSource) (ordinal 0) (authored-target "focus::image")
        (range (start 22 12) (end 22 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::image") (range (start 17 3) (end 17 18)))
        )
      )
    )
    (query (range (start 22 27) (end 22 38)) (probe (position 22 27))
      (reference
        (source (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))
        (kind flowTarget) (ordinal 0) (authored-target "shoot::image")
        (range (start 22 27) (end 22 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image") (range (start 25 3) (end 25 17)))
        )
      )
    )
  )
)
~~~
