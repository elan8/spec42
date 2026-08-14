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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 17) (end 3 38))
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
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction out))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image::isWellFocused"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Boolean"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction in))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "focus::image::isWellFocused"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0)))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "shoot"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot"))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image::isWellFocused"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "focus::image::isWellFocused")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image::isWellFocused")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0)))))) (kind thenTarget) (ordinal 0))
      (authored-target "shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::image"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)))))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image::isWellFocused"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0)))))) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0)))))) (kind thenTarget) (ordinal 0)))
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
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 7 50) (end 7 55)) (probe (position 7 50))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 7 31) (end 7 36)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 3 17) (end 3 38)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image::isWellFocused"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 8 30) (end 8 35)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 8 51) (end 8 58)) (probe (position 8 51))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 9 58) (end 9 65)) (probe (position 9 58))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Picture")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 9 37) (end 9 42)) (probe (position 9 37))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Scene")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 11 22) (end 11 33)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::TakePicture")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 20 5) (end 20 30)) (probe (position 20 5))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "focus::image::isWellFocused")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::Image::isWellFocused")))))
  )
  (query (document "memory://snapshot/16_conditional_succession_example_2.md") (range (start 20 36) (end 20 41)) (probe (position 20 36))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_2.md") (path (name "Conditional Succession Example-2") (name "takePicture") (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0)))))) (kind thenTarget) (ordinal 0) (authored-target "shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_2.md") (qualified-name "Conditional Succession Example-2::takePicture::shoot")))))
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
