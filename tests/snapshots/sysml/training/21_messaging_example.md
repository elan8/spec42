# META
~~~ini
description=SysML Training 21 (Asynchronous Messaging): Messaging Example
type=file
~~~
# SOURCE
~~~sysml
package 'Messaging Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	attribute def Show {
		item picture : Picture;
	}
	
	action def Focus { in item scene : Scene; out item image : Image; }
	action def Shoot { in item image : Image; out item picture : Picture; }
	action def TakePicture;
	
	action screen;
		
	action takePicture : TakePicture {
		action trigger accept scene : Scene;
		
		then action focus : Focus {
			in item scene = trigger.scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot : Shoot {
			in item image; 
			out item picture;
		}
		
		then send new Show(shoot.picture) to screen;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/21_messaging_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5d3cc1a64a4746f4f3f2ad75fe010be12e6751fdc0e69b332edbb9cb6c2ab8c7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show::picture"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::screen"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "focus::image")) (flowTarget (reference "shoot::image")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "shoot::picture")) (invocationCallee (reference "Show")) (sendTarget (reference "screen")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::trigger"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "Scene")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "focus::image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus::image")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "shoot::image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot::image")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "shoot::picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Show")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind sendTarget) (ordinal 0))
      (authored-target "screen")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::screen")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::trigger"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show::picture"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus::image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::screen"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind sendTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::trigger"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::trigger"))) (kind acceptPayloadType) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image")) (scopes any))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture")) (scopes any))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show::picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show::picture")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")))
      (subtype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture")))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus::image")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus")))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus::scene")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus")))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture")))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture")))
      (type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot::image")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot")))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot::picture")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot")))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::trigger")))
      (featured-by (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 9 60) (end 9 65)) (probe (position 9 60))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 9 36) (end 9 41)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 10 36) (end 10 41)) (probe (position 10 36))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 10 62) (end 10 69)) (probe (position 10 62))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 6 17) (end 6 24)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 15 22) (end 15 33)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 23 12) (end 23 23)) (probe (position 23 12))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "focus::image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus::image")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 23 27) (end 23 38)) (probe (position 23 27))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (path (named (kind package) (name "Messaging Example")) (named (kind action) (name "takePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "shoot::image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot::image")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 18 22) (end 18 27)) (probe (position 18 22))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 30 21) (end 30 34)) (probe (position 30 21))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind memberAccessOperand) (ordinal 0) (authored-target "shoot::picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 30 16) (end 30 20)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind invocationCallee) (ordinal 0) (authored-target "Show")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 30 39) (end 30 45)) (probe (position 30 39))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::send"))) (kind sendTarget) (ordinal 0) (authored-target "screen")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::screen")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 25 22) (end 25 27)) (probe (position 25 22))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 16 32) (end 16 37)) (probe (position 16 32))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::trigger"))) (kind acceptPayloadType) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")))))
    )
  )
)
~~~
