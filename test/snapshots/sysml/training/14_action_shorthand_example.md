# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Shorthand Example
type=file
~~~
# SOURCE
~~~sysml
package 'Action Shorthand Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		action focus: Focus {
			in item scene = TakePicture::scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot: Shoot {
			in item;
			out item picture = TakePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_shorthand_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 13 3) (end 13 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 27) (end 17 38))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 21 3) (end 21 43))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Shorthand Example' {
    item def Scene;
    item def Image;
    item def Picture;

    action def Focus { in scene : Scene; out image : Image; }
    action def Shoot { in image: Image; out picture : Picture; }

    action def TakePicture {
        in item scene : Scene;
        out item picture : Picture;

        action focus: Focus {
            in item scene = TakePicture::scene;
            out item image;
        }

        flow from focus.image to shoot.image;

        then action shoot: Shoot {
            in item;
            out item picture = TakePicture::picture;
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ec5238d27706142e3e82d3a51def27a5d470e49986befda432b41e643aed5ecb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example"))) (kind "package") (name "Action Shorthand Example") (declared-name "Action Shorthand Example"))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (parent (node (document "d0") (qualified-name "Action Shorthand Example"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Shorthand Example::Focus"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Shorthand Example::Focus"))) (authored (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Image"))) (kind "item def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "Action Shorthand Example"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Picture"))) (kind "item def") (name "Picture") (declared-name "Picture") (parent (node (document "d0") (qualified-name "Action Shorthand Example"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Scene"))) (kind "item def") (name "Scene") (declared-name "Scene") (parent (node (document "d0") (qualified-name "Action Shorthand Example"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (parent (node (document "d0") (qualified-name "Action Shorthand Example"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Shorthand Example::Shoot"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Shorthand Example::Shoot"))) (authored (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (parent (node (document "d0") (qualified-name "Action Shorthand Example"))) (authored (membership (kind Owning)) (relationships (perform (reference "Action Shorthand Example::TakePicture::focus")) (perform (reference "Action Shorthand Example::TakePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (authored (relationships (typing (reference "Shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (kind performSource) (ordinal 0)) (authored-target "Action Shorthand Example::TakePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (kind performSource) (ordinal 1)) (authored-target "Action Shorthand Example::TakePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Shorthand Example::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::Focus::image"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::Focus::scene"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::image"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::picture"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (target (node (document "d0") (qualified-name "Action Shorthand Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::scene")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot::picture")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 12) (end 17 23)) (probe (position 17 12))
      (reference
        (source (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))
        (kind flowSource) (ordinal 0) (authored-target "focus::image")
        (range (start 17 12) (end 17 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::image") (range (start 14 3) (end 14 18)))
        )
      )
    )
    (query (range (start 17 27) (end 17 38)) (probe (position 17 27))
      (reference
        (source (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))
        (kind flowTarget) (ordinal 0) (authored-target "shoot::image")
        (range (start 17 27) (end 17 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
