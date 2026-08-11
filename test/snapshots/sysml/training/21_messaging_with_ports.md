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
    (element (id (node (document "d0") (qualified-name "Messaging Example"))) (kind "package") (name "Messaging Example") (declared-name "Messaging Example") (range (start (line 0) (character 0)) (end (line 0) (character 779))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 9) (character 1)) (end (line 9) (character 68))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 9) (character 43)) (end (line 9) (character 66))) (parent (node (document "d0") (qualified-name "Messaging Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 9) (character 20)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "Messaging Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Image"))) (kind "item def") (name "Image") (declared-name "Image") (range (start (line 2) (character 1)) (end (line 2) (character 16))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Picture"))) (kind "item def") (name "Picture") (declared-name "Picture") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Scene"))) (kind "item def") (name "Scene") (declared-name "Scene") (range (start (line 1) (character 1)) (end (line 1) (character 16))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 10) (character 1)) (end (line 10) (character 72))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 10) (character 20)) (end (line 10) (character 42))) (parent (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 10) (character 43)) (end (line 10) (character 70))) (parent (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::Show"))) (kind "attribute def") (name "Show") (declared-name "Show") (range (start (line 5) (character 1)) (end (line 5) (character 50))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (range (start (line 11) (character 1)) (end (line 11) (character 24))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera"))) (kind "part") (name "camera") (declared-name "camera") (range (start (line 17) (character 1)) (end (line 17) (character 430))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::displayPort"))) (kind "port") (name "displayPort") (declared-name "displayPort") (range (start (line 19) (character 2)) (end (line 19) (character 19))) (parent (node (document "d0") (qualified-name "Messaging Example::camera"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (range (start (line 21) (character 2)) (end (line 21) (character 372))) (parent (node (document "d0") (qualified-name "Messaging Example::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture") (range none)) (perform (reference "Messaging Example::camera::takePicture::focus") (range none)) (perform (reference "Messaging Example::camera::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 24) (character 3)) (end (line 24) (character 90))) (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (authored (relationships (typing (reference "Focus") (range none)) (flow (reference "Messaging Example::camera::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 26) (character 4)) (end (line 26) (character 19))) (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 25) (character 4)) (end (line 25) (character 34))) (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 29) (character 3)) (end (line 29) (character 40))) (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 31) (character 3)) (end (line 31) (character 77))) (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (authored (relationships (typing (reference "Shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 32) (character 4)) (end (line 32) (character 18))) (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 33) (character 4)) (end (line 33) (character 21))) (parent (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::camera::viewPort"))) (kind "port") (name "viewPort") (declared-name "viewPort") (range (start (line 18) (character 2)) (end (line 18) (character 16))) (parent (node (document "d0") (qualified-name "Messaging Example::camera"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::screen"))) (kind "part") (name "screen") (declared-name "screen") (range (start (line 13) (character 1)) (end (line 13) (character 37))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::screen::displayPort"))) (kind "port") (name "displayPort") (declared-name "displayPort") (range (start (line 14) (character 2)) (end (line 14) (character 19))) (parent (node (document "d0") (qualified-name "Messaging Example::screen"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (range (start (line 29) (character 13)) (end (line 29) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (range (start (line 29) (character 28)) (end (line 29) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Messaging Example::camera::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Messaging Example::camera::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (kind flowSource) (ordinal 0)) (authored-target "Messaging Example::camera::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
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
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image"))) (target (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "focus::image") (target "shoot::image") (source-range (start (line 29) (character 13)) (end (line 29) (character 24))) (target-range (start (line 29) (character 28)) (end (line 29) (character 39)))))
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
