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
  (document "21_messaging_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 19 3) (end 19 33))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 30 2) (end 30 48))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "9c29ea77d7152372c9bd61c370d7bc67d4131adb3f622e5a9e603378333b3ecc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Messaging Example"))) (kind "package") (name "Messaging Example") (declared-name "Messaging Example") (range (start (line 0) (character 0)) (end (line 0) (character 665))))
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
    (element (id (node (document "d0") (qualified-name "Messaging Example::screen"))) (kind "action") (name "screen") (declared-name "screen") (range (start (line 13) (character 1)) (end (line 13) (character 15))) (parent (node (document "d0") (qualified-name "Messaging Example"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (range (start (line 15) (character 1)) (end (line 15) (character 337))) (parent (node (document "d0") (qualified-name "Messaging Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture") (range none)) (perform (reference "Messaging Example::takePicture::trigger") (range none)) (perform (reference "Messaging Example::takePicture::focus") (range none)) (perform (reference "Messaging Example::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 18) (character 2)) (end (line 18) (character 86))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (authored (relationships (typing (reference "Focus") (range none)) (flow (reference "Messaging Example::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 20) (character 3)) (end (line 20) (character 18))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 19) (character 3)) (end (line 19) (character 33))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 23) (character 2)) (end (line 23) (character 39))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 25) (character 2)) (end (line 25) (character 73))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (authored (relationships (typing (reference "Shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 26) (character 3)) (end (line 26) (character 17))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 27) (character 3)) (end (line 27) (character 20))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Messaging Example::takePicture::trigger"))) (kind "action") (name "trigger") (declared-name "trigger") (range (start (line 16) (character 2)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "Messaging Example::takePicture"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (range (start (line 23) (character 12)) (end (line 23) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (range (start (line 23) (character 27)) (end (line 23) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Messaging Example::takePicture::trigger") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::trigger")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Messaging Example::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind performSource) (ordinal 2)) (authored-target "Messaging Example::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (kind flowSource) (ordinal 0)) (authored-target "Messaging Example::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (target (node (document "d0") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (target (node (document "d0") (qualified-name "Messaging Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (target (node (document "d0") (qualified-name "Messaging Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (target (node (document "d0") (qualified-name "Messaging Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (target (node (document "d0") (qualified-name "Messaging Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::trigger"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (target (node (document "d0") (qualified-name "Messaging Example::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::image"))) (target (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "focus::image") (target "shoot::image") (source-range (start (line 23) (character 12)) (end (line 23) (character 23))) (target-range (start (line 23) (character 27)) (end (line 23) (character 38)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::scene")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 23 12) (end 23 23)) (probe (position 23 12))
      (reference
        (source (document "d0") (qualified-name "Messaging Example::takePicture"))
        (kind flowSource) (ordinal 0) (authored-target "focus::image")
        (range (start 23 12) (end 23 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Messaging Example::takePicture::focus::image") (range (start 20 3) (end 20 18)))
        )
      )
    )
    (query (range (start 23 27) (end 23 38)) (probe (position 23 27))
      (reference
        (source (document "d0") (qualified-name "Messaging Example::takePicture"))
        (kind flowTarget) (ordinal 0) (authored-target "shoot::image")
        (range (start 23 27) (end 23 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Messaging Example::takePicture::shoot::image") (range (start 26 3) (end 26 17)))
        )
      )
    )
  )
)
~~~
