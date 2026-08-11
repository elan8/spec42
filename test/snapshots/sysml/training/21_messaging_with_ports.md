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
  (document "21_messaging_with_ports.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 25 4) (end 25 34))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "34add96ae32c0f39cd163387412a9d9e471bde35bdae87095cc3824b3edc5b1c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Messaging Example"))) (kind "package") (name "Messaging Example") (declared-name "Messaging Example"))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Messaging Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Messaging Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Image"))) (kind "item def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Picture"))) (kind "item def") (name "Picture") (declared-name "Picture") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Scene"))) (kind "item def") (name "Scene") (declared-name "Scene") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Show"))) (kind "attribute def") (name "Show") (declared-name "Show") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera"))) (kind "part") (name "camera") (declared-name "camera") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::displayPort"))) (kind "port") (name "displayPort") (declared-name "displayPort") (parent (node (document "d0") (qualified-name "Messaging Example::camera"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (parent (node (document "d0") (qualified-name "Messaging Example::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture")) (perform (reference "Messaging Example::camera::takePicture::focus")) (perform (reference "Messaging Example::camera::takePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (authored (relationships (typing (reference "Focus")) (flow (reference "Messaging Example::camera::takePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (authored (relationships (typing (reference "Shoot")))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::viewPort"))) (kind "port") (name "viewPort") (declared-name "viewPort") (parent (node (document "d0") (qualified-name "Messaging Example::camera"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::screen"))) (kind "part") (name "screen") (declared-name "screen") (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::screen::displayPort"))) (kind "port") (name "displayPort") (declared-name "displayPort") (parent (node (document "d0") (qualified-name "Messaging Example::screen"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Messaging Example::camera::takePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Messaging Example::camera::takePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind flowSource) (ordinal 0)) (authored-target "Messaging Example::camera::takePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (target (node (document "d0") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (target (node (document "d0") (qualified-name "Messaging Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (target (node (document "d0") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (target (node (document "d0") (qualified-name "Messaging Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (target (node (document "d0") (qualified-name "Messaging Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (target (node (document "d0") (qualified-name "Messaging Example::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image"))) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "focus::image") (target "shoot::image")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::scene")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 29 13) (end 29 24)) (probe (position 29 13))
      (reference
        (source (document "d0") (qualified-name "Messaging Example::camera::takePicture"))
        (kind flowSource) (ordinal 0) (authored-target "focus::image")
        (range (start 29 13) (end 29 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image") (range (start 26 4) (end 26 19)))
        )
      )
    )
    (query (range (start 29 28) (end 29 39)) (probe (position 29 28))
      (reference
        (source (document "d0") (qualified-name "Messaging Example::camera::takePicture"))
        (kind flowTarget) (ordinal 0) (authored-target "shoot::image")
        (range (start 29 28) (end 29 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image") (range (start 32 4) (end 32 18)))
        )
      )
    )
  )
)
~~~
