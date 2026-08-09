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

        action focus : Focus {
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
(model
  (namespace
    (package 'Action Shorthand Example'
      (item_def 'Scene')
      (item_def 'Image')
      (item_def 'Picture')
      (action_def 'Focus'
        (reference_usage in reference 'scene' : 'Action Shorthand Example::Scene'[item_def])
        (reference_usage out reference 'image' : 'Action Shorthand Example::Image'[item_def]))
      (action_def 'Shoot'
        (reference_usage in reference 'image' : 'Action Shorthand Example::Image'[item_def])
        (reference_usage out reference 'picture' : 'Action Shorthand Example::Picture'[item_def]))
      (action_def 'TakePicture'
        (item_usage in 'scene' : 'Action Shorthand Example::Scene'[item_def])
        (item_usage out 'picture' : 'Action Shorthand Example::Picture'[item_def])
        (action_usage composite 'focus' : 'Action Shorthand Example::Focus'[action_def]
          (item_usage in 'scene'
            (feature_value (=)))
          (item_usage out 'image'))
        (flow_usage composite
          (connector_end 'focus.image')
          (connector_end 'shoot.image'))
        (source_succession
          (action_usage 'shoot' : 'Action Shorthand Example::Shoot'[action_def]
            (item_usage in)
            (item_usage out 'picture'
              (feature_value (=)))))))))
~~~
