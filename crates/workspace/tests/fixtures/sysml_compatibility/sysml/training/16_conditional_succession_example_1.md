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
        isWellFocused : ScalarValues::Boolean;
    }
    part def Picture;

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
    }

    action takePicture : TakePicture {
        in item scene;
        out item picture;

        action focus : Focus {
            in item scene = takePicture::scene;
            out item image;
        }

        first focus;
        if focus.image.isWellFocused;
        then shoot;

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
(model
  (namespace
    (package 'Conditional Succession Example-1'
      (part_def 'Scene')
      (part_def 'Image'
        (reference_usage reference 'isWellFocused' : 'ScalarValues::Boolean'[unresolved]))
      (part_def 'Picture')
      (action_def 'Focus'
        (reference_usage in reference 'scene' : 'Conditional Succession Example-1::Scene'[part_def])
        (reference_usage out reference 'image' : 'Conditional Succession Example-1::Image'[part_def]))
      (action_def 'Shoot'
        (reference_usage in reference 'image' : 'Conditional Succession Example-1::Image'[part_def])
        (reference_usage out reference 'picture' : 'Conditional Succession Example-1::Picture'[part_def]))
      (action_def 'TakePicture'
        (reference_usage in reference 'scene' : 'Conditional Succession Example-1::Scene'[part_def])
        (reference_usage out reference 'picture' : 'Conditional Succession Example-1::Picture'[part_def]))
      (action_usage 'takePicture' : 'Conditional Succession Example-1::TakePicture'[action_def]
        (item_usage in 'scene')
        (item_usage out 'picture')
        (action_usage composite 'focus' : 'Conditional Succession Example-1::Focus'[action_def]
          (item_usage in 'scene'
            (feature_value (=)))
          (item_usage out 'image'))
        (initial_node)
        (if_action_usage)
        (source_succession
          (reference_usage reference 'shoot'))
        (flow_usage composite
          (connector_end 'focus.image')
          (connector_end 'shoot.image'))
        (action_usage composite 'shoot' : 'Conditional Succession Example-1::Shoot'[action_def]
          (item_usage in)
          (item_usage out 'picture'
            (feature_value (=))))))))
~~~
