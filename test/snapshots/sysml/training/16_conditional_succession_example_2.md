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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c745f466697de1b101bf5534b38f55260c2b1d5ebddbf3a5da217f99f387a587") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2"))) (kind "package") (name "Conditional Succession Example-2") (declared-name "Conditional Succession Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 689))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 7) (character 1)) (end (line 7) (character 58))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 7) (character 38)) (end (line 7) (character 56))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 7) (character 20)) (end (line 7) (character 37))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Image"))) (kind "part def") (name "Image") (declared-name "Image") (range (start (line 2) (character 1)) (end (line 2) (character 60))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture"))) (kind "part def") (name "Picture") (declared-name "Picture") (range (start (line 5) (character 1)) (end (line 5) (character 18))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene"))) (kind "part def") (name "Scene") (declared-name "Scene") (range (start (line 1) (character 1)) (end (line 1) (character 16))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 8) (character 1)) (end (line 8) (character 61))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 8) (character 20)) (end (line 8) (character 36))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 8) (character 37)) (end (line 8) (character 59))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (range (start (line 9) (character 1)) (end (line 9) (character 68))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 9) (character 44)) (end (line 9) (character 66))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 9) (character 26)) (end (line 9) (character 43))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (range (start (line 11) (character 1)) (end (line 11) (character 348))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture") (range none)) (perform (reference "Conditional Succession Example-2::takePicture::focus") (range none)) (perform (reference "Conditional Succession Example-2::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 15) (character 2)) (end (line 15) (character 87))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 17) (character 3)) (end (line 17) (character 18))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 16) (character 3)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 22) (character 2)) (end (line 22) (character 39))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 13) (character 2)) (end (line 13) (character 19))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 12) (character 2)) (end (line 12) (character 16))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 24) (character 2)) (end (line 24) (character 91))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 25) (character 3)) (end (line 25) (character 17))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 26) (character 3)) (end (line 26) (character 43))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (range (start (line 22) (character 12)) (end (line 22) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (range (start (line 22) (character 27)) (end (line 22) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Conditional Succession Example-2::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Conditional Succession Example-2::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::Shoot")))))
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
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::focus::image"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "focus::image") (target "shoot::image") (source-range (start (line 22) (character 12)) (end (line 22) (character 23))) (target-range (start (line 22) (character 27)) (end (line 22) (character 38)))))
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
