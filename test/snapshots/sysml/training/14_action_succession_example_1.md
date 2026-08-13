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
  (document "memory://snapshot/14_action_succession_example_1.md"
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
        (range (start 12 2) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 14 24) (end 14 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 14 34) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 20 24) (end 20 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 20 34) (end 20 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 22 2) (end 22 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d5673eaabcadc6ae55b994aad4e9a600c78efd7948dcd053da47adc42c9a7bf9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Image"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Picture"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Scene"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture"))))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene"))))
    (declaration (id (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Focus")))))
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Scene")))))
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14_action_succession_example_1.md") (range (start 14 16) (end 14 21)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Focus")))))
  )
  (query (document "memory://snapshot/14_action_succession_example_1.md") (range (start 10 21) (end 10 28)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Picture")))))
  )
  (query (document "memory://snapshot/14_action_succession_example_1.md") (range (start 9 18) (end 9 23)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Scene")))))
  )
  (query (document "memory://snapshot/14_action_succession_example_1.md") (range (start 20 16) (end 20 21)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_succession_example_1.md") (qualified-name "Action Succession Example-1::Shoot")))))
  )
)
~~~
