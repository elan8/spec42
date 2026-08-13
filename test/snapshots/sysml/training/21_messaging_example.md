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
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 6 2) (end 6 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 18 2) (end 21 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 23 2) (end 23 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 25 2) (end 28 3))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 30 2) (end 31 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:5d3cc1a64a4746f4f3f2ad75fe010be12e6751fdc0e69b332edbb9cb6c2ab8c7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image"))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene"))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image"))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture"))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Show"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::screen"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture"))))
    (declaration (id (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture::trigger"))) (kind action) (membership (kind feature) (visibility default)))
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
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 9 36) (end 9 41)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Scene")))))
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 10 36) (end 10 41)) (probe (position 10 36))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Image")))))
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 10 62) (end 10 69)) (probe (position 10 62))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::Picture")))))
  )
  (query (document "memory://snapshot/21_messaging_example.md") (range (start 15 22) (end 15 33)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_example.md") (qualified-name "Messaging Example::TakePicture")))))
  )
)
~~~
