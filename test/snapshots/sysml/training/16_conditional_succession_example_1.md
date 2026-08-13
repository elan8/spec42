# META
~~~ini
description=SysML Training 16 (Conditional Succession): Conditional Succession Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Conditional Succession Example-1' {
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
				
		first focus 
			if focus.image.isWellFocused then shoot;
		
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
  (document "memory://snapshot/16_conditional_succession_example_1.md"
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
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 20 2) (end 23 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 23 2) (end 23 39))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 26 3) (end 27 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:fa723e0804b7453f4c8d0c1ac6c6501a874727670f2a266f133f18b7809c1a78") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::picture"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::scene"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 11 22) (end 11 33)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 15 17) (end 15 22)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 25 17) (end 25 22)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")))))
  )
)
~~~
