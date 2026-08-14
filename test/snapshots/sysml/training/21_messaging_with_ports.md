# META
~~~ini
description=SysML Training 21 (Asynchronous Messaging): Messaging with Ports
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
	
	part screen {
		port displayPort;
	}
	
	part camera {
		port viewPort;
		port displayPort;
		
		action takePicture : TakePicture {
			action trigger accept scene : Scene via viewPort;
			
			then action focus : Focus {
				in item scene = trigger.scene;
				out item image;
			}
			
			flow from focus.image to shoot.image;
		
			then action shoot : Shoot {
				in item image; 
				out item picture;
			}
			
			then send new Show(shoot.picture) via displayPort;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/21_messaging_with_ports.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 29 3) (end 29 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 36 3) (end 37 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:feb91c879d2638e3bb574cfb56398fb3cc26ba87e69fea1960c2a020c15da8f3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show::picture"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::displayPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (acceptVia (reference "viewPort")) (acceptPayloadType (reference "Scene")))))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::viewPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::screen"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::screen::displayPort"))) (kind port) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (kind acceptVia) (ordinal 0))
      (authored-target "viewPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::viewPort")))))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::image"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::scene"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::image"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::picture"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show::picture"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind acceptVia) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::viewPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (kind acceptVia) (ordinal 0)))
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (kind acceptPayloadType) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus")))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::image")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::scene")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::image")) (scopes any))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::picture")) (scopes any))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show::picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot")))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::image")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::picture")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show::picture")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture")))
      (subtype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot")))
      (type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot")) (source direct))
      (supertype (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 9 60) (end 9 65)) (probe (position 9 60))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 9 36) (end 9 41)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 10 36) (end 10 41)) (probe (position 10 36))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Image")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 10 62) (end 10 69)) (probe (position 10 62))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 6 17) (end 6 24)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Show::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Picture")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 21 23) (end 21 34)) (probe (position 21 23))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 24 23) (end 24 28)) (probe (position 24 23))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Focus")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 31 23) (end 31 28)) (probe (position 31 23))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Shoot")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 22 43) (end 22 51)) (probe (position 22 43))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (kind acceptVia) (ordinal 0) (authored-target "viewPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::viewPort")))))
    )
  )
  (query (document "memory://snapshot/21_messaging_with_ports.md") (range (start 22 33) (end 22 38)) (probe (position 22 33))
    (reference (id (source (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::camera::takePicture::trigger"))) (kind acceptPayloadType) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/21_messaging_with_ports.md") (qualified-name "Messaging Example::Scene")))))
    )
  )
)
~~~
