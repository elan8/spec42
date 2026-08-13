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
  (document "memory://snapshot/16_conditional_succession_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 3 2) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 7 20) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 7 38) (end 7 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 8 20) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 8 37) (end 8 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 9 26) (end 9 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 9 44) (end 9 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 20 2) (end 20 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 22 2) (end 22 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d12745b09f40a6fabe37f90af438c499c4bf267a9a3ba9fa67a3574860aafa36") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::picture"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::scene"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 11 22) (end 11 33)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 15 17) (end 15 22)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 24 17) (end 24 22)) (probe (position 24 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot")))))
  )
)
~~~
