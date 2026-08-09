# META
~~~ini
description=SysML Training 15 (Actions): Action Decomposition
type=file
~~~
# SOURCE
~~~sysml
package 'Action Decomposition' {
	part def Scene;
	part def Image;
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
KwPart,KwDef,Ident,Semicolon,
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
  (package_def ''Action Decomposition''
    (part_def 'Scene')
    (part_def 'Image')
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
      (flow_usage
        (connector_end)
        (connector_end))
      (action_usage 'shoot' : 'Shoot'
        (item_usage in)
        (item_usage out 'picture' value)))))
~~~
# FORMAT
~~~sysml
package 'Action Decomposition' {
    part def Scene;
    part def Image;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Action Decomposition"))) (name "Action Decomposition") (declared-name "Action Decomposition")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (name "Focus") (declared-name "Focus")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::Focus")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (name "scene") (declared-name "scene") (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::Focus")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Action Decomposition::Image"))) (name "Image") (declared-name "Image") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Action Decomposition::Picture"))) (name "Picture") (declared-name "Picture") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Action Decomposition::Scene"))) (name "Scene") (declared-name "Scene") (declared))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::Shoot")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (name "picture") (declared-name "picture") (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::Shoot")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))) (name "TakePicture") (declared-name "TakePicture")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (name "picture") (declared-name "picture") (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::TakePicture")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (name "scene") (declared-name "scene") (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::TakePicture")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (name "takePicture") (declared-name "takePicture") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (name "focus") (declared-name "focus") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::image"))) (name "image") (declared-name "image") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::Focus")))))
                (element (kind "item") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in") (composite true) (reference false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "takePicture::scene")))) (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::Focus"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus::scene"))) (role feature-value))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::TakePicture")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (name "shoot") (declared-name "shoot") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))))
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out") (composite true) (reference false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "takePicture::picture")))) (effective (featuring-type (node (document "d0") (qualified-name "Action Decomposition::Shoot"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot::picture"))) (role feature-value))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (to (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (to (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::Focus::image"))) (to (node (document "d0") (qualified-name "Action Decomposition::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::Focus::scene"))) (to (node (document "d0") (qualified-name "Action Decomposition::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::Shoot::image"))) (to (node (document "d0") (qualified-name "Action Decomposition::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::Shoot::picture"))) (to (node (document "d0") (qualified-name "Action Decomposition::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::TakePicture::picture"))) (to (node (document "d0") (qualified-name "Action Decomposition::Picture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::TakePicture::scene"))) (to (node (document "d0") (qualified-name "Action Decomposition::Scene"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::takePicture"))) (to (node (document "d0") (qualified-name "Action Decomposition::TakePicture"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::takePicture::focus"))) (to (node (document "d0") (qualified-name "Action Decomposition::Focus"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Decomposition::takePicture::shoot"))) (to (node (document "d0") (qualified-name "Action Decomposition::Shoot"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
