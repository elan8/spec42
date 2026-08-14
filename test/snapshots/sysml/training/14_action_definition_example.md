# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Definition Example
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
		
	action def TakePicture { in scene : Scene; out picture : Picture;
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/14_action_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 13 2) (end 13 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:09ac2c25b2ccabae242821ffc66b9fef46f1dcfe1c7e2b0ff4222b33e9f10336") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::image"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction out))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Image"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Picture"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Scene"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::image"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction in))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "scene")) (memberAccessOperand (reference "focus::scene"))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "picture")) (memberAccessOperand (reference "shoot::picture"))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot"))))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (kind parameter) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Scene")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0))
      (authored-target "picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "focus::scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "shoot::picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Scene")))))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::image"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::image"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::focus"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::shoot"))) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 5 50) (end 5 55)) (probe (position 5 50))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Image")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 5 31) (end 5 36)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Scene")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 6 30) (end 6 35)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Image")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 6 51) (end 6 58)) (probe (position 6 51))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Picture")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 9 21) (end 9 26)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 17 23) (end 17 30)) (probe (position 17 23))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0) (authored-target "picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 9 7) (end 9 18)) (probe (position 9 7))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "focus::scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus::scene")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 17 7) (end 17 20)) (probe (position 17 7))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "shoot::picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot::picture")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 11 16) (end 11 21)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Focus")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 8 58) (end 8 65)) (probe (position 8 58))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Picture")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 8 37) (end 8 42)) (probe (position 8 37))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Scene")))))
  )
  (query (document "memory://snapshot/14_action_definition_example.md") (range (start 15 16) (end 15 21)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_definition_example.md") (qualified-name "Action Definition Example::Shoot")))))
  )
)
~~~
