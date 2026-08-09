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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwMerge,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Merge Example''
    (part_def 'Scene')
    (part_def 'Image')
    (part_def 'Picture')
    (action_def 'Focus'
      (item_usage in 'scene' : 'Scene')
      (item_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (item_usage in 'image' : 'Image')
      (item_usage out 'picture' : 'Picture'))
    (action_def 'Display'
      (item_usage in 'picture' : 'Picture'))
    (action_def 'TakePicture')
    (action_usage 'takePicture' : 'TakePicture'
      (initial_node start)
      (source_succession
        (sysml_decl 'continue'))
      (source_succession
        (action_usage 'trigger'
          (item_usage out 'scene' : 'Scene')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'focus' : 'Focus'
          (item_usage in 'scene')
          (item_usage out 'image')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'shoot' : 'Shoot'
          (item_usage in 'image')
          (item_usage out 'picture')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'display' : 'Display'
          (item_usage in 'picture')))
      (source_succession
        (default_ref_usage 'continue')))))
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
# EXPECTED
~~~
semantic.duplicate_name 'continue'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'continue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Merge Example"))) (name "Merge Example") (declared-name "Merge Example")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "Merge Example::Display"))) (name "Display") (declared-name "Display")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Display")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Merge Example::Focus"))) (name "Focus") (declared-name "Focus")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Focus")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Focus")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Merge Example::Image"))) (name "Image") (declared-name "Image") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Merge Example::Picture"))) (name "Picture") (declared-name "Picture") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Merge Example::Scene"))) (name "Scene") (declared-name "Scene") (declared))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Merge Example::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (name "image") (declared-name "image") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Shoot")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Shoot")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Merge Example::TakePicture"))) (name "TakePicture") (declared-name "TakePicture"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Merge Example::takePicture"))) (name "takePicture") (declared-name "takePicture") (declared)
          (contains
            (element (kind "initial") (id (node (document "d0") (qualified-name "Merge Example::takePicture::_initial"))) (name "_initial") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
            (element (kind "merge") (id (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (name "merge") (declared-name "merge") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (name "display") (declared-name "display") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::takePicture::display::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Display")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (name "focus") (declared-name "focus") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Focus")))))
                (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Focus")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Merge Example::takePicture::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
            (element (kind "flow") (id (node (document "d0") (qualified-name "Merge Example::takePicture::from#flow"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
            (element (kind "flow") (id (node (document "d0") (qualified-name "Merge Example::takePicture::from#flow2"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (name "shoot") (declared-name "shoot") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::image"))) (name "image") (declared-name "image") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Shoot")))))
                (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::Shoot")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::display"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::display"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture"))) (to (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (to (node (document "d0") (qualified-name "Merge Example::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (to (node (document "d0") (qualified-name "Merge Example::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (to (node (document "d0") (qualified-name "Merge Example::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (to (node (document "d0") (qualified-name "Merge Example::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (to (node (document "d0") (qualified-name "Merge Example::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture"))) (to (node (document "d0") (qualified-name "Merge Example::TakePicture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (to (node (document "d0") (qualified-name "Merge Example::Display"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (to (node (document "d0") (qualified-name "Merge Example::Focus"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (to (node (document "d0") (qualified-name "Merge Example::Shoot"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (to (node (document "d0") (qualified-name "Merge Example::Scene"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "Merge Example::takePicture::_initial") (target-qualified "Merge Example::takePicture::start"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/17_merge_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
    )
  )
)
~~~
