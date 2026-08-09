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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwPort,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,KwAccept,Ident,Colon,Ident,KwVia,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwThen,KwSend,Ident,Ident,OpenParen,Ident,Dot,Ident,CloseParen,KwVia,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Messaging Example''
    (item_def 'Scene')
    (item_def 'Image')
    (item_def 'Picture')
    (attribute_def 'Show'
      (item_usage 'picture' : 'Picture'))
    (action_def 'Focus'
      (item_usage in 'scene' : 'Scene')
      (item_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (item_usage in 'image' : 'Image')
      (item_usage out 'picture' : 'Picture'))
    (action_def 'TakePicture')
    (part_usage 'screen'
      (port_usage 'displayPort'))
    (part_usage 'camera'
      (port_usage 'viewPort')
      (port_usage 'displayPort')
      (action_usage 'takePicture' : 'TakePicture'
        (action_usage 'trigger')
        (accept_node)
        (source_succession
          (action_usage 'focus' : 'Focus'
            (item_usage in 'scene' value)
            (item_usage out 'image')))
        (flow_usage
          (connector_end)
          (connector_end))
        (source_succession
          (action_usage 'shoot' : 'Shoot'
            (item_usage in 'image')
            (item_usage out 'picture')))
        (source_succession
          (send_node))))))
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Messaging Example"))) (name "Messaging Example") (declared-name "Messaging Example")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "Messaging Example::Focus"))) (name "Focus") (declared-name "Focus")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Focus")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Focus")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "Messaging Example::Image"))) (name "Image") (declared-name "Image"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Messaging Example::Picture"))) (name "Picture") (declared-name "Picture"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Messaging Example::Scene"))) (name "Scene") (declared-name "Scene"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (name "image") (declared-name "image") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Messaging Example::Show"))) (name "Show") (declared-name "Show") (declared (properties (ordered false) (unique true))))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Messaging Example::TakePicture"))) (name "TakePicture") (declared-name "TakePicture"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Messaging Example::camera"))) (name "camera") (declared-name "camera") (declared (properties (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Messaging Example::camera::displayPort"))) (name "displayPort") (declared-name "displayPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (name "takePicture") (declared-name "takePicture") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (name "focus") (declared-name "focus") (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::TakePicture"))))
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Focus")))))
                    (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in")) (feature-value (kind bound) (expression (kind "memberAccess") (reference "scene") (children (expression (kind "featureReference") (reference "trigger")))))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Focus"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus::scene"))) (role feature-value))))
                  )
                )
                (element (kind "flow") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::TakePicture")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (name "shoot") (declared-name "shoot") (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::TakePicture"))))
                  (contains
                    (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::image"))) (name "image") (declared-name "image") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
                    (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
                  )
                )
              )
            )
            (element (kind "port") (id (node (document "d0") (qualified-name "Messaging Example::camera::viewPort"))) (name "viewPort") (declared-name "viewPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Messaging Example::screen"))) (name "screen") (declared-name "screen") (declared (properties (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Messaging Example::screen::displayPort"))) (name "displayPort") (declared-name "displayPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (to (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (to (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (to (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (to (node (document "d0") (qualified-name "Messaging Example::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (to (node (document "d0") (qualified-name "Messaging Example::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (to (node (document "d0") (qualified-name "Messaging Example::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (to (node (document "d0") (qualified-name "Messaging Example::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::camera::takePicture"))) (to (node (document "d0") (qualified-name "Messaging Example::TakePicture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::focus"))) (to (node (document "d0") (qualified-name "Messaging Example::Focus"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::camera::takePicture::shoot"))) (to (node (document "d0") (qualified-name "Messaging Example::Shoot"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/21_messaging_with_ports.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 14 2) (end 14 19))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 18 2) (end 18 16))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 19 2) (end 19 19))
      )
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
