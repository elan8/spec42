# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Succession Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Action Succession Example-1' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
		first focus then shoot;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_succession_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 12 7) (end 12 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 24) (end 14 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 34) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 24) (end 20 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 34) (end 20 46))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 22 7) (end 22 20))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e2384668817074c362790529b8dd6d56c72e480b663cf9c327204b50c26a2df5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1"))) (kind "package") (name "Action Succession Example-1") (declared-name "Action Succession Example-1"))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (authored (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Image"))) (kind "item def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Picture"))) (kind "item def") (name "Picture") (declared-name "Picture") (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Scene"))) (kind "item def") (name "Scene") (declared-name "Scene") (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (authored (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (parent (node (document "d0") (qualified-name "Action Succession Example-1"))) (authored (membership (kind Owning)) (relationships (perform (reference "Action Succession Example-1::TakePicture::focus")) (perform (reference "Action Succession Example-1::TakePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (authored (relationships (typing (reference "")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 0)) (authored-target "focus::scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 3)) (authored-target "shoot::picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindTarget) (ordinal 0)) (authored-target "scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindTarget) (ordinal 3)) (authored-target "picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 1)) (authored-target "focus::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 2)) (authored-target "focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowTarget) (ordinal 1)) (authored-target "shoot::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowTarget) (ordinal 2)) (authored-target "shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 0)) (authored-target "Action Succession Example-1::TakePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 1)) (authored-target "Action Succession Example-1::TakePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "focus") (target "shoot")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "focus::image") (target "shoot::image")))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "focus::scene") (target "scene")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 3)) (expression (kind bind) (source "shoot::picture") (target "picture")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 21) (end 12 26)) (probe (position 12 21))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind bindTarget) (ordinal 0) (authored-target "scene")
        (range (start 12 21) (end 12 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene") (range (start 9 2) (end 9 24)))
        )
      )
    )
    (query (range (start 18 8) (end 18 13)) (probe (position 18 8))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind flowSource) (ordinal 2) (authored-target "focus")
        (range (start 18 8) (end 18 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus") (range (start 14 2) (end 14 46)))
        )
      )
    )
    (query (range (start 18 19) (end 18 24)) (probe (position 18 19))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind flowTarget) (ordinal 2) (authored-target "shoot")
        (range (start 18 19) (end 18 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot") (range (start 20 2) (end 20 48)))
        )
      )
    )
    (query (range (start 22 23) (end 22 30)) (probe (position 22 23))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind bindTarget) (ordinal 3) (authored-target "picture")
        (range (start 22 23) (end 22 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture") (range (start 10 2) (end 10 29)))
        )
      )
    )
    (query (range (start 12 7) (end 12 18)) (probe (position 12 7))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind bindSource) (ordinal 0) (authored-target "focus::scene")
        (range (start 12 7) (end 12 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene") (range (start 14 24) (end 14 33)))
        )
      )
    )
    (query (range (start 16 12) (end 16 23)) (probe (position 16 12))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind flowSource) (ordinal 1) (authored-target "focus::image")
        (range (start 16 12) (end 16 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image") (range (start 14 34) (end 14 44)))
        )
      )
    )
    (query (range (start 16 27) (end 16 38)) (probe (position 16 27))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind flowTarget) (ordinal 1) (authored-target "shoot::image")
        (range (start 16 27) (end 16 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image") (range (start 20 24) (end 20 33)))
        )
      )
    )
    (query (range (start 22 7) (end 22 20)) (probe (position 22 7))
      (reference
        (source (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))
        (kind bindSource) (ordinal 3) (authored-target "shoot::picture")
        (range (start 22 7) (end 22 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture") (range (start 20 34) (end 20 46)))
        )
      )
    )
  )
)
~~~
