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
    (element (id (node (document "d0") (qualified-name "Action Decomposition"))) (kind "package") (name "Action Decomposition") (declared-name "Action Decomposition") (range (start (line 0) (character 0)) (end (line 0) (character 582))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 5) (character 1)) (end (line 5) (character 58))) (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 5) (character 38)) (end (line 5) (character 56))) (parent (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 5) (character 20)) (end (line 5) (character 37))) (parent (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Image"))) (kind "part def") (name "Image") (declared-name "Image") (range (start (line 2) (character 1)) (end (line 2) (character 16))) (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Picture"))) (kind "part def") (name "Picture") (declared-name "Picture") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Scene"))) (kind "part def") (name "Scene") (declared-name "Scene") (range (start (line 1) (character 1)) (end (line 1) (character 16))) (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 6) (character 1)) (end (line 6) (character 61))) (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 6) (character 20)) (end (line 6) (character 36))) (parent (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 6) (character 37)) (end (line 6) (character 59))) (parent (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (range (start (line 7) (character 1)) (end (line 7) (character 68))) (parent (node (document "d0") (qualified-name "Action Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 7) (character 44)) (end (line 7) (character 66))) (parent (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 7) (character 26)) (end (line 7) (character 43))) (parent (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (range (start (line 9) (character 1)) (end (line 9) (character 296))) (parent (node (document "d0") (qualified-name "Action Decomposition"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture") (range none)) (perform (reference "Action Decomposition::takePicture::focus") (range none)) (perform (reference "Action Decomposition::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 13) (character 2)) (end (line 13) (character 87))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 15) (character 3)) (end (line 15) (character 18))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 14) (character 3)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 18) (character 2)) (end (line 18) (character 39))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 11) (character 2)) (end (line 11) (character 19))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 10) (character 2)) (end (line 10) (character 16))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 20) (character 2)) (end (line 20) (character 85))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 22) (character 3)) (end (line 22) (character 43))) (parent (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (range (start (line 18) (character 12)) (end (line 18) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (range (start (line 18) (character 27)) (end (line 18) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Action Decomposition::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Action Decomposition::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Decomposition::Shoot")))))
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
