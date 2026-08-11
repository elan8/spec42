# META
~~~ini
description=SysML Training 17 (Control): Merge Example
type=file
~~~
# SOURCE
~~~sysml
package 'Merge Example' {
	part def Scene;
	part def Image;
	part def Picture;
	
	action def Focus { in item scene : Scene; out item image : Image; }
	action def Shoot { in item image : Image; out item picture : Picture; }
	action def Display { in item picture : Picture; }
	action def TakePicture;
	
	action takePicture : TakePicture {
		first start;
		
		then merge continue;
			
		then action trigger {
			out item scene : Scene;
		}
		
		flow from trigger.scene to focus.scene;
		
		then action focus : Focus {
			in item scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot : Shoot {
			in item image ;
			out item picture;
		}
		
		flow from shoot.picture to display.picture;
		
		then action display : Display {
			in item picture;
		}
		
		then continue;	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_merge_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 2) (end 11 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 54))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Merge Example' {
    part def Scene;
    part def Image;
    part def Picture;

    action def Focus { in item scene : Scene; out item image : Image; }
    action def Shoot { in item image : Image; out item picture : Picture; }
    action def Display { in item picture : Picture; }
    action def TakePicture;

    action takePicture : TakePicture {
        first start;

        then merge continue;

        then action trigger {
            out item scene : Scene;
        }

        flow from trigger.scene to focus.scene;

        then action focus : Focus {
            in item scene;
            out item image;
        }

        flow from focus.image to shoot.image;

        then action shoot : Shoot {
            in item image ;
            out item picture;
        }

        flow from shoot.picture to display.picture;

        then action display : Display {
            in item picture;
        }

        then continue;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1b2617aebe36b634a1f4f52111539f2bfbf19ae5feb8e9075ed12c0bd149e127") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Merge Example"))) (kind "package") (name "Merge Example") (declared-name "Merge Example"))
    (element (id (node (document "d0") (qualified-name "Merge Example::Display"))) (kind "action def") (name "Display") (declared-name "Display") (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Merge Example::Display"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Merge Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Merge Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Image"))) (kind "part def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Picture"))) (kind "part def") (name "Picture") (declared-name "Picture") (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Scene"))) (kind "part def") (name "Scene") (declared-name "Scene") (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Merge Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Merge Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (parent (node (document "d0") (qualified-name "Merge Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture")) (perform (reference "Merge Example::takePicture::trigger")) (perform (reference "Merge Example::takePicture::focus")) (perform (reference "Merge Example::takePicture::shoot")) (perform (reference "Merge Example::takePicture::display")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::_initial"))) (kind "initial") (name "_initial") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (flow (reference "Merge Example::takePicture::start")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (kind "merge") (name "merge") (declared-name "merge") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (flow (reference "Merge Example::takePicture::trigger")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind "action") (name "display") (declared-name "display") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "Display")) (flow (reference "Merge Example::takePicture::continue")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::display::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Merge Example::takePicture::display"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "Focus")) (flow (reference "Merge Example::takePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::from#flow"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::from#flow2"))) (kind "flow") (name "from") (declared-name "from") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "Shoot")) (flow (reference "Merge Example::takePicture::display")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::image"))) (kind "item") (name "image") (declared-name "image") (parent (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (parent (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind "action") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "")) (flow (reference "Merge Example::takePicture::focus")))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (kind "item") (name "scene") (declared-name "scene") (parent (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "trigger::scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 1)) (authored-target "focus::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 2)) (authored-target "shoot::picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "focus::scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowTarget) (ordinal 1)) (authored-target "shoot::image") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowTarget) (ordinal 2)) (authored-target "display::picture") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Merge Example::takePicture::trigger") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Merge Example::takePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 2)) (authored-target "Merge Example::takePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 3)) (authored-target "Merge Example::takePicture::display") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::trigger") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind featureTyping) (ordinal 0)) (authored-target "Display") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Display")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::continue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::continue")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::display") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::focus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Scene")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (target (node (document "d0") (qualified-name "Merge Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (target (node (document "d0") (qualified-name "Merge Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (target (node (document "d0") (qualified-name "Merge Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (target (node (document "d0") (qualified-name "Merge Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (target (node (document "d0") (qualified-name "Merge Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (target (node (document "d0") (qualified-name "Merge Example::Display"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (target (node (document "d0") (qualified-name "Merge Example::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus::image"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "focus::image") (target "shoot::image")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Merge Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display::picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "shoot::picture") (target "display::picture")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (target (node (document "d0") (qualified-name "Merge Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus::scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "trigger::scene") (target "focus::scene")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 19 29) (end 19 40)) (probe (position 19 29))
      (reference
        (source (document "d0") (qualified-name "Merge Example::takePicture"))
        (kind flowTarget) (ordinal 0) (authored-target "focus::scene")
        (range (start 19 29) (end 19 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Merge Example::takePicture::focus::scene") (range (start 22 3) (end 22 17)))
        )
      )
    )
    (query (range (start 26 12) (end 26 23)) (probe (position 26 12))
      (reference
        (source (document "d0") (qualified-name "Merge Example::takePicture"))
        (kind flowSource) (ordinal 1) (authored-target "focus::image")
        (range (start 26 12) (end 26 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Merge Example::takePicture::focus::image") (range (start 23 3) (end 23 18)))
        )
      )
    )
    (query (range (start 26 27) (end 26 38)) (probe (position 26 27))
      (reference
        (source (document "d0") (qualified-name "Merge Example::takePicture"))
        (kind flowTarget) (ordinal 1) (authored-target "shoot::image")
        (range (start 26 27) (end 26 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Merge Example::takePicture::shoot::image") (range (start 29 3) (end 29 18)))
        )
      )
    )
    (query (range (start 19 12) (end 19 25)) (probe (position 19 12))
      (reference
        (source (document "d0") (qualified-name "Merge Example::takePicture"))
        (kind flowSource) (ordinal 0) (authored-target "trigger::scene")
        (range (start 19 12) (end 19 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene") (range (start 16 3) (end 16 26)))
        )
      )
    )
    (query (range (start 33 12) (end 33 25)) (probe (position 33 12))
      (reference
        (source (document "d0") (qualified-name "Merge Example::takePicture"))
        (kind flowSource) (ordinal 2) (authored-target "shoot::picture")
        (range (start 33 12) (end 33 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture") (range (start 30 3) (end 30 20)))
        )
      )
    )
    (query (range (start 33 29) (end 33 44)) (probe (position 33 29))
      (reference
        (source (document "d0") (qualified-name "Merge Example::takePicture"))
        (kind flowTarget) (ordinal 2) (authored-target "display::picture")
        (range (start 33 29) (end 33 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Merge Example::takePicture::display::picture") (range (start 36 3) (end 36 19)))
        )
      )
    )
  )
)
~~~
