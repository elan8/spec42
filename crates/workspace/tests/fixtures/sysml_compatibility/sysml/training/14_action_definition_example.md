# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Definition Example
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
		
	action def TakePicture { in scene : Scene; out picture : Picture;
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
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
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
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
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'picture' : 'Picture')
      (binding_as_usage
        (connector_end)
        (connector_end))
      (action_usage 'focus' : 'Focus'
        (default_ref_usage in 'scene')
        (default_ref_usage out 'image'))
      (flow_usage
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

    action def Focus {
        in scene : Scene;
        out image : Image;
    }
    action def Shoot {
        in image : Image;
        out picture : Picture;
    }

    action def TakePicture {
        in scene : Scene;
        out picture : Picture;
        bind focus.scene = scene;

        action focus : Focus {
            in scene;
            out image;
        }

        flow from focus.image to shoot.image;

        action shoot : Shoot {
            in image;
            out picture;
        }

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
(model
  (namespace
    (package 'Action Definition Example'
      (item_def 'Scene')
      (item_def 'Image')
      (item_def 'Picture')
      (action_def 'Focus'
        (reference_usage in reference 'scene' : 'Action Definition Example::Scene'[item_def])
        (reference_usage out reference 'image' : 'Action Definition Example::Image'[item_def]))
      (action_def 'Shoot'
        (reference_usage in reference 'image' : 'Action Definition Example::Image'[item_def])
        (reference_usage out reference 'picture' : 'Action Definition Example::Picture'[item_def]))
      (action_def 'TakePicture'
        (reference_usage in reference 'scene' : 'Action Definition Example::Scene'[item_def])
        (reference_usage out reference 'picture' : 'Action Definition Example::Picture'[item_def])
        (binding_connector_def
          (connector_end 'focus.scene')
          (connector_end 'scene'))
        (action_usage composite 'focus' : 'Action Definition Example::Focus'[action_def]
          (reference_usage in reference 'scene')
          (reference_usage out reference 'image'))
        (flow_usage composite
          (connector_end 'focus.image')
          (connector_end 'shoot.image'))
        (action_usage composite 'shoot' : 'Action Definition Example::Shoot'[action_def]
          (reference_usage in reference 'image')
          (reference_usage out reference 'picture'))
        (binding_connector_def
          (connector_end 'shoot.picture')
          (connector_end 'picture'))))))
~~~
