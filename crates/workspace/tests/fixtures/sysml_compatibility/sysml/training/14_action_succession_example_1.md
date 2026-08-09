# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Succession Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Action Succession Example-1' {
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
		
		flow from focus.image to shoot.image;
		
		first focus then shoot;
		
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
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Succession Example-1''
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
      (flow_usage
        (connector_end)
        (connector_end))
      (succession_as_usage
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
package 'Action Succession Example-1' {
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
        in item scene : Scene;
        out item picture : Picture;

        bind focus.scene = scene;

        action focus : Focus {
            in scene;
            out image;
        }

        flow from focus.image to shoot.image;

        first focus then shoot;

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
    (package 'Action Succession Example-1'
      (item_def 'Scene')
      (item_def 'Image')
      (item_def 'Picture')
      (action_def 'Focus'
        (reference_usage in reference 'scene' : 'Action Succession Example-1::Scene'[item_def])
        (reference_usage out reference 'image' : 'Action Succession Example-1::Image'[item_def]))
      (action_def 'Shoot'
        (reference_usage in reference 'image' : 'Action Succession Example-1::Image'[item_def])
        (reference_usage out reference 'picture' : 'Action Succession Example-1::Picture'[item_def]))
      (action_def 'TakePicture'
        (item_usage in 'scene' : 'Action Succession Example-1::Scene'[item_def])
        (item_usage out 'picture' : 'Action Succession Example-1::Picture'[item_def])
        (binding_connector_def
          (connector_end 'focus.scene')
          (connector_end 'scene'))
        (action_usage composite 'focus' : 'Action Succession Example-1::Focus'[action_def]
          (reference_usage in reference 'scene')
          (reference_usage out reference 'image'))
        (flow_usage composite
          (connector_end 'focus.image')
          (connector_end 'shoot.image'))
        (succession_def
          (connector_end 'focus')
          (connector_end 'shoot'))
        (action_usage composite 'shoot' : 'Action Succession Example-1::Shoot'[action_def]
          (reference_usage in reference 'image')
          (reference_usage out reference 'picture'))
        (binding_connector_def
          (connector_end 'shoot.picture')
          (connector_end 'picture'))))))
~~~
