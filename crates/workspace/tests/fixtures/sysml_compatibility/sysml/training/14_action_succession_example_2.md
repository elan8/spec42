# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Succession Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Action Definition Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		succession flow from focus.image to shoot.image;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
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
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwSuccession,KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Definition Example''
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
      (binding_as_usage
        (connector_end)
        (connector_end))
      (action_usage 'focus' : 'Focus'
        (default_ref_usage in 'scene')
        (default_ref_usage out 'image'))
      (succession_flow_usage
        (connector_end)
        (connector_end))
      (action_usage 'shoot' : 'Shoot'
        (default_ref_usage in 'image')
        (default_ref_usage out 'picture'))
      (binding_as_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Action Definition Example' {
    item def Scene;
    item def Image;
    item def Picture;

    action def Focus { in scene : Scene; out image : Image; }
    action def Shoot { in image: Image; out picture : Picture; }

    action def TakePicture {
        in item scene : Scene;
        out item picture : Picture;

        bind focus.scene = scene;

        action focus: Focus { in scene; out image; }

        succession flow from focus.image to shoot.image;

        action shoot: Shoot { in image; out picture; }

        bind shoot.picture = picture;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Action Definition Example"))) (name "Action Definition Example") (declared-name "Action Definition Example")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (name "Focus") (declared-name "Focus")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Focus")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (name "scene") (declared-name "scene") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Focus")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "Action Definition Example::Image"))) (name "Image") (declared-name "Image"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (name "Picture") (declared-name "Picture"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (name "Scene") (declared-name "Scene"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (name "Shoot") (declared-name "Shoot")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Shoot")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (name "picture") (declared-name "picture") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Shoot")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (name "TakePicture") (declared-name "TakePicture")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (name "focus") (declared-name "focus") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Focus")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (name "scene") (declared-name "scene") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Focus")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::from"))) (name "from") (declared-name "from") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (name "picture") (declared-name "picture") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::TakePicture")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (name "scene") (declared-name "scene") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::TakePicture")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (name "shoot") (declared-name "shoot") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (name "image") (declared-name "image") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Shoot")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (name "picture") (declared-name "picture") (effective (featuring-type (node (document "d0") (qualified-name "Action Definition Example::Shoot")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (bind (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (to (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (connect (source-expression "focus::scene") (target-expression "scene") (container-prefix "Action Definition Example::TakePicture")) (provenance authored))
    (bind (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (to (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (connect (source-expression "shoot::picture") (target-expression "picture") (container-prefix "Action Definition Example::TakePicture")) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (to (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (to (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (to (node (document "d0") (qualified-name "Action Definition Example::Image"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (to (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (to (node (document "d0") (qualified-name "Action Definition Example::Image"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (to (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (to (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (to (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (to (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (to (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::Image"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::from"))) (status missing-prerequisite) (target "Flows::messages"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/14_action_succession_example_2.md"
    (diagnostics
    )
  )
)
~~~
