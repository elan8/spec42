# META
~~~ini
description=SysML Training 16 (Conditional Succession): Conditional Succession Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Conditional Succession Example-1' {
	part def Scene;
	part def Image {
		isWellFocused: ScalarValues::Boolean;
	}
	part def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
	action def TakePicture { in scene : Scene; out picture : Picture; }
	
	action takePicture : TakePicture {
		in item scene;
		out item picture;
		
		action focus : Focus {
			in item scene = takePicture::scene; 
			out item image;
		}
				
		first focus 
			if focus.image.isWellFocused then shoot;
		
		flow from focus.image to shoot.image;

		action shoot : Shoot {
			in item; 
			out item picture = takePicture::picture;
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFirst,Ident,
KwIf,Ident,Dot,Ident,Dot,Ident,KwThen,Ident,Semicolon,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Semicolon,
KwOut,KwItem,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Conditional Succession Example-1''
    (part_def 'Scene')
    (part_def 'Image'
      (default_ref_usage 'isWellFocused' : 'ScalarValues::Boolean'))
    (part_def 'Picture')
    (action_def 'Focus'
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (default_ref_usage in 'image' : 'Image')
      (default_ref_usage out 'picture' : 'Picture'))
    (action_def 'TakePicture'
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'picture' : 'Picture'))
    (action_usage 'takePicture' : 'TakePicture'
      (item_usage in 'scene')
      (item_usage out 'picture')
      (action_usage 'focus' : 'Focus'
        (item_usage in 'scene' value)
        (item_usage out 'image'))
      (initial_node focus)
      (if_node)
      (source_succession
        (default_ref_usage 'shoot'))
      (flow_usage
        (connector_end)
        (connector_end))
      (action_usage 'shoot' : 'Shoot'
        (item_usage in)
        (item_usage out 'picture' value)))))
~~~
# FORMAT
~~~sysml
package 'Conditional Succession Example-1' {
    part def Scene;
    part def Image {
        isWellFocused: ScalarValues::Boolean;
    }
    part def Picture;

    action def Focus { in scene : Scene; out image : Image; }
    action def Shoot { in image: Image; out picture : Picture; }
    action def TakePicture { in scene : Scene; out picture : Picture; }

    action takePicture : TakePicture {
        in item scene;
        out item picture;

        action focus : Focus {
            in item scene = takePicture::scene;
            out item image;
        }

        first focus
        if focus.image.isWellFocused then shoot;

        flow from focus.image to shoot.image;

        action shoot : Shoot {
            in item;
            out item picture = takePicture::picture;
        }
    }

}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'shoot'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'shoot'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Conditional Succession Example-1"))) (name "Conditional Succession Example-1") (declared-name "Conditional Succession Example-1")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus"))) (name "Focus") (declared-name "Focus")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (name "scene") (declared-name "scene") (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Image"))) (name "Image") (declared-name "Image") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture"))) (name "Picture") (declared-name "Picture") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene"))) (name "Scene") (declared-name "Scene") (declared))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (name "picture") (declared-name "picture") (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))) (name "TakePicture") (declared-name "TakePicture")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (name "picture") (declared-name "picture") (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (name "scene") (declared-name "scene") (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (name "takePicture") (declared-name "takePicture") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (name "focus") (declared-name "focus") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus")))))
                (element (kind "item") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in") (composite true) (reference false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "takePicture::scene")))) (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus::scene"))) (role feature-value))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (name "shoot") (declared-name "shoot") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out") (composite true) (reference false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "takePicture::picture")))) (effective (featuring-type (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot::picture"))) (role feature-value))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::image"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (to (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot"))))
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
  (document "sysml/training/16_conditional_succession_example_1.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 16 3) (end 16 38))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 20 2) (end 20 64))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 26 3) (end 26 16))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 27 3) (end 27 43))
      )
    )
  )
)
~~~
