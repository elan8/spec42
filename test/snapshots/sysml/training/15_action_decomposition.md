# META
~~~ini
description=SysML Training 15 (Actions): Action Decomposition
type=file
~~~
# SOURCE
~~~sysml
package 'Action Decomposition' {
	part def Scene;
	part def Image;
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
		
		flow from focus.image to shoot.image;

		action shoot : Shoot {
			in item; 
			out item picture = takePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_action_decomposition.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 14 3) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 27) (end 18 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 21 3) (end 21 16))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 22 3) (end 22 43))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Decomposition' {
    part def Scene;
    part def Image;
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

        flow from focus.image to shoot.image;

        action shoot : Shoot {
            in item;
            out item picture = takePicture::picture;
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "189c7f32d91e0af4b92920ea6e779d640f435ac325dbf31512c66b91cd236e05") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Action Decomposition"))) (kind "package") (name "Action Decomposition") (declared-name "Action Decomposition"))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (authored (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Image"))) (kind "part def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Picture"))) (kind "part def") (name "Picture") (declared-name "Picture") (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Scene"))) (kind "part def") (name "Scene") (declared-name "Scene") (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (authored (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (authored (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (authored (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (parent (node (document "d0") (qualified-name "Action Decomposition"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture")) (perform (reference "Action Decomposition::takePicture::focus")) (perform (reference "Action Decomposition::takePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Action Decomposition::takePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Action Decomposition::takePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (target (node (document "d0") (qualified-name "Action Decomposition::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (target (node (document "d0") (qualified-name "Action Decomposition::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (target (node (document "d0") (qualified-name "Action Decomposition::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (target (node (document "d0") (qualified-name "Action Decomposition::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Action Decomposition::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Action Decomposition::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (target (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (target (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::scene")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot::picture")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 18 12) (end 18 23)) (probe (position 18 12))
      (reference
        (source (document "d0") (qualified-name "Action Decomposition::takePicture"))
        (kind flowSource) (ordinal 0) (authored-target "focus::image")
        (range (start 18 12) (end 18 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Decomposition::takePicture::focus::image") (range (start 15 3) (end 15 18)))
        )
      )
    )
    (query (range (start 18 27) (end 18 38)) (probe (position 18 27))
      (reference
        (source (document "d0") (qualified-name "Action Decomposition::takePicture"))
        (kind flowTarget) (ordinal 0) (authored-target "shoot::image")
        (range (start 18 27) (end 18 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
