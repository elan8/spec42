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
KwAction,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,KwAccept,Ident,Colon,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwThen,KwSend,Ident,Ident,OpenParen,Ident,Dot,Ident,CloseParen,KwTo,Ident,Semicolon,
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
    (action_usage 'screen')
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
        (send_node)))))
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
        (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::screen"))) (name "screen") (declared-name "screen") (declared))
        (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (name "takePicture") (declared-name "takePicture") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (name "focus") (declared-name "focus") (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Focus")))))
                (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in")) (feature-value (kind bound) (expression (kind "memberAccess") (reference "scene") (children (expression (kind "featureReference") (reference "trigger")))))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Focus"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::scene"))) (role feature-value))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::TakePicture")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (name "shoot") (declared-name "shoot") (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::image"))) (name "image") (declared-name "image") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
                (element (kind "item") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Messaging Example::Shoot")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Messaging Example::takePicture::trigger"))) (name "trigger") (declared-name "trigger") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Messaging Example::TakePicture")))))
          )
        )
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (to (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (to (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (to (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (to (node (document "d0") (qualified-name "Messaging Example::takePicture::trigger"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (to (node (document "d0") (qualified-name "Messaging Example::Image"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (to (node (document "d0") (qualified-name "Messaging Example::Scene"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (to (node (document "d0") (qualified-name "Messaging Example::Image"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (to (node (document "d0") (qualified-name "Messaging Example::Picture"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (to (node (document "d0") (qualified-name "Messaging Example::TakePicture"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (to (node (document "d0") (qualified-name "Messaging Example::Focus"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (to (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Focus"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Focus::image"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Focus::scene"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Image"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Picture"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Scene"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Shoot"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Shoot::image"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Shoot::picture"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::Show"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::TakePicture"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::screen"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::focus"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::image"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::focus::scene"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::from"))) (status missing-prerequisite) (target "Flows::messages"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::image"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::shoot::picture"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Messaging Example::takePicture::trigger"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/21_messaging_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "accept_payload_incompatible")
        (source "semantic")
        (range (start 16 2) (end 16 38))
      )
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
