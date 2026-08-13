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
  (document "memory://snapshot/14_action_shorthand_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 5 20) (end 5 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 5 38) (end 5 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 6 20) (end 6 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 6 37) (end 6 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 17 2) (end 17 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 19 2) (end 22 3))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 20 3) (end 21 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:6409babd991a20b892be47396abad82d5e8da9f8e41b377e6a10f72342e58bb0") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture"))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 12 16) (end 12 21)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")))))
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 10 21) (end 10 28)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")))))
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 9 18) (end 9 23)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")))))
  )
)
~~~
