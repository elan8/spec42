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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 17) (end 3 38))
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
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::image"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction out)))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in)))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image::isWellFocused"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Boolean")))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction in)))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out)))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out)))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in)))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture")))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus")))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot")))))
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image::isWellFocused"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")))))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")))))
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
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::image"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::image")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::scene")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::image")) (scopes any))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image::isWellFocused")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::picture")) (scopes any))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::scene")) (scopes any))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::image")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::picture")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")))
      (subtype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::picture")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::scene")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus::image")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus")))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus::scene")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus")))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::picture")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture")))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::scene")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture")))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture")))
      (type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")) (source direct))
      (supertype (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot::picture")))
      (featured-by (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 7 50) (end 7 55)) (probe (position 7 50))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 7 31) (end 7 36)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 3 17) (end 3 38)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image::isWellFocused"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 8 30) (end 8 35)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Image")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 8 51) (end 8 58)) (probe (position 8 51))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 9 58) (end 9 65)) (probe (position 9 58))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Picture")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 9 37) (end 9 42)) (probe (position 9 37))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Scene")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 11 22) (end 11 33)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 15 17) (end 15 22)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Focus")))))
    )
  )
  (query (document "memory://snapshot/16_conditional_succession_example_1.md") (range (start 25 17) (end 25 22)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/16_conditional_succession_example_1.md") (qualified-name "Conditional Succession Example-1::Shoot")))))
    )
  )
)
~~~
