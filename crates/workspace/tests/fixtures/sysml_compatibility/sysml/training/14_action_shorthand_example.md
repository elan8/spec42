# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Shorthand Example
type=file
~~~
# SOURCE
~~~sysml
package 'Action Shorthand Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		action focus: Focus {
			in item scene = TakePicture::scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot: Shoot {
			in item;
			out item picture = TakePicture::picture;
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
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Semicolon,
KwOut,KwItem,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Shorthand Example''
    (item_def 'Scene')
    (item_def 'Image')
    (item_def 'Picture')
    (action_def 'Focus'
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (default_ref_usage in 'image' : 'Image')
      (default_ref_usage out 'picture' : 'Picture'))
    (action_def 'TakePicture'
      (item_usage in 'scene' : 'Scene')
      (item_usage out 'picture' : 'Picture')
      (action_usage 'focus' : 'Focus'
        (item_usage in 'scene' value)
        (item_usage out 'image'))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'shoot' : 'Shoot'
          (item_usage in)
          (item_usage out 'picture' value))))))
~~~
# FORMAT
~~~sysml
package 'Action Shorthand Example' {
    item def Scene;
    item def Image;
    item def Picture;

    action def Focus { in scene : Scene; out image : Image; }
    action def Shoot { in image: Image; out picture : Picture; }

    action def TakePicture {
        in item scene : Scene;
        out item picture : Picture;

        action focus: Focus {
            in item scene = TakePicture::scene;
            out item image;
        }

        flow from focus.image to shoot.image;

        then action shoot: Shoot {
            in item;
            out item picture = TakePicture::picture;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Action Shorthand Example"))) (name "Action Shorthand Example") (declared-name "Action Shorthand Example")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Shorthand Example::Focus"))) (name "Focus") (declared-name "Focus")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Shorthand Example::Focus::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::Focus")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Shorthand Example::Focus::scene"))) (name "scene") (declared-name "scene") (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::Focus")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "Action Shorthand Example::Image"))) (name "Image") (declared-name "Image"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Action Shorthand Example::Picture"))) (name "Picture") (declared-name "Picture"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Action Shorthand Example::Scene"))) (name "Scene") (declared-name "Scene"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Shorthand Example::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::Shoot")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::picture"))) (name "picture") (declared-name "picture") (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::Shoot")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (name "TakePicture") (declared-name "TakePicture")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (name "focus") (declared-name "focus") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::Focus")))))
                (element (kind "item") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in")) (feature-value (kind bound) (expression (kind "featureReference") (reference "TakePicture::scene")))) (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::Focus"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus::scene"))) (role feature-value))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (name "shoot") (declared-name "shoot") (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out")) (feature-value (kind bound) (expression (kind "featureReference") (reference "TakePicture::picture")))) (effective (featuring-type (node (document "d0") (qualified-name "Action Shorthand Example::Shoot"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot::picture"))) (role feature-value))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::Focus::image"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::Focus::scene"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::image"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::Shoot::picture"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Focus"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (to (node (document "d0") (qualified-name "Action Shorthand Example::Shoot"))))
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
  (document "sysml/training/14_action_shorthand_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 13 3) (end 13 38))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 21 3) (end 21 43))
      )
    )
  )
)
~~~
