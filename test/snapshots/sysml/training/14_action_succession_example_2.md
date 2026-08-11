# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Succession Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Action Definition Example' {
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
		
		succession flow from focus.image to shoot.image;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_succession_example_2.md"
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
        (range (start 18 24) (end 18 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 34) (end 18 46))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 20 7) (end 20 20))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "9d8dca45241100c382dab3228b1f3e0f080c0df647b6594db215b027cce5a31c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Action Definition Example"))) (kind "package") (name "Action Definition Example") (declared-name "Action Definition Example"))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (authored (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Image"))) (kind "item def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (kind "item def") (name "Picture") (declared-name "Picture") (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (kind "item def") (name "Scene") (declared-name "Scene") (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (authored (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (authored (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (parent (node (document "d0") (qualified-name "Action Definition Example"))) (authored (membership (kind Owning)) (relationships (perform (reference "Action Definition Example::TakePicture::focus")) (perform (reference "Action Definition Example::TakePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (authored (relationships (typing (reference "")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 0)) (authored-target "focus::scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 2)) (authored-target "shoot::picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindTarget) (ordinal 0)) (authored-target "scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindTarget) (ordinal 2)) (authored-target "picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind successionFlowSource) (ordinal 1)) (authored-target "focus::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind successionFlowTarget) (ordinal 1)) (authored-target "shoot::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 0)) (authored-target "Action Definition Example::TakePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 1)) (authored-target "Action Definition Example::TakePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (target (node (document "d0") (qualified-name "Action Definition Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (target (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (target (node (document "d0") (qualified-name "Action Definition Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (target (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (target (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind successionFlow) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind successionFlowSource) (ordinal 1)) (expression (kind successionFlow) (source "focus::image") (target "shoot::image")))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "focus::scene") (target "scene")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (target (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 2)) (expression (kind bind) (source "shoot::picture") (target "picture")))
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
        (source (document "d0") (qualified-name "Action Definition Example::TakePicture"))
        (kind bindTarget) (ordinal 0) (authored-target "scene")
        (range (start 12 21) (end 12 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Definition Example::TakePicture::scene") (range (start 9 2) (end 9 24)))
        )
      )
    )
    (query (range (start 20 23) (end 20 30)) (probe (position 20 23))
      (reference
        (source (document "d0") (qualified-name "Action Definition Example::TakePicture"))
        (kind bindTarget) (ordinal 2) (authored-target "picture")
        (range (start 20 23) (end 20 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Definition Example::TakePicture::picture") (range (start 10 2) (end 10 29)))
        )
      )
    )
    (query (range (start 12 7) (end 12 18)) (probe (position 12 7))
      (reference
        (source (document "d0") (qualified-name "Action Definition Example::TakePicture"))
        (kind bindSource) (ordinal 0) (authored-target "focus::scene")
        (range (start 12 7) (end 12 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene") (range (start 14 24) (end 14 33)))
        )
      )
    )
    (query (range (start 16 23) (end 16 34)) (probe (position 16 23))
      (reference
        (source (document "d0") (qualified-name "Action Definition Example::TakePicture"))
        (kind successionFlowSource) (ordinal 1) (authored-target "focus::image")
        (range (start 16 23) (end 16 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image") (range (start 14 34) (end 14 44)))
        )
      )
    )
    (query (range (start 16 38) (end 16 49)) (probe (position 16 38))
      (reference
        (source (document "d0") (qualified-name "Action Definition Example::TakePicture"))
        (kind successionFlowTarget) (ordinal 1) (authored-target "shoot::image")
        (range (start 16 38) (end 16 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image") (range (start 18 24) (end 18 33)))
        )
      )
    )
    (query (range (start 20 7) (end 20 20)) (probe (position 20 7))
      (reference
        (source (document "d0") (qualified-name "Action Definition Example::TakePicture"))
        (kind bindSource) (ordinal 2) (authored-target "shoot::picture")
        (range (start 20 7) (end 20 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture") (range (start 18 34) (end 18 46)))
        )
      )
    )
  )
)
~~~
