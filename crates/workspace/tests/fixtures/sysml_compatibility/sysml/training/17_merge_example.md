# META
~~~ini
description=SysML Training 17 (Control): Merge Example
type=file
~~~
# SOURCE
~~~sysml
package 'Merge Example' {
	part def Scene;
	part def Image;
	part def Picture;
	
	action def Focus { in item scene : Scene; out item image : Image; }
	action def Shoot { in item image : Image; out item picture : Picture; }
	action def Display { in item picture : Picture; }
	action def TakePicture;
	
	action takePicture : TakePicture {
		first start;
		
		then merge continue;
			
		then action trigger {
			out item scene : Scene;
		}
		
		flow from trigger.scene to focus.scene;
		
		then action focus : Focus {
			in item scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot : Shoot {
			in item image ;
			out item picture;
		}
		
		flow from shoot.picture to display.picture;
		
		then action display : Display {
			in item picture;
		}
		
		then continue;	
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwMerge,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Merge Example''
    (part_def 'Scene')
    (part_def 'Image')
    (part_def 'Picture')
    (action_def 'Focus'
      (item_usage in 'scene' : 'Scene')
      (item_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (item_usage in 'image' : 'Image')
      (item_usage out 'picture' : 'Picture'))
    (action_def 'Display'
      (item_usage in 'picture' : 'Picture'))
    (action_def 'TakePicture')
    (action_usage 'takePicture' : 'TakePicture'
      (initial_node start)
      (source_succession
        (sysml_decl 'continue'))
      (source_succession
        (action_usage 'trigger'
          (item_usage out 'scene' : 'Scene')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'focus' : 'Focus'
          (item_usage in 'scene')
          (item_usage out 'image')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'shoot' : 'Shoot'
          (item_usage in 'image')
          (item_usage out 'picture')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'display' : 'Display'
          (item_usage in 'picture')))
      (source_succession
        (default_ref_usage 'continue')))))
~~~
# FORMAT
~~~sysml
package 'Merge Example' {
    part def Scene;
    part def Image;
    part def Picture;

    action def Focus {
        in item scene : Scene;
        out item image : Image;
    }
    action def Shoot {
        in item image : Image;
        out item picture : Picture;
    }
    action def Display {
        in item picture : Picture;
    }
    action def TakePicture;

    action takePicture : TakePicture {
        first start;

        then merge continue;

        then action trigger {
			out item scene : Scene;
		}

        flow from trigger.scene to focus.scene;

        then action focus : Focus {
			in item scene;
			out item image;
		}

        flow from focus.image to shoot.image;

        then action shoot : Shoot {
			in item image ;
			out item picture;
		}

        flow from shoot.picture to display.picture;

        then action display : Display {
			in item picture;
		}

        then continue;
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'continue'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'continue'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Merge Example'
      (part_def 'Scene')
      (part_def 'Image')
      (part_def 'Picture')
      (action_def 'Focus'
        (item_usage in 'scene' : 'Merge Example::Scene'[part_def])
        (item_usage out 'image' : 'Merge Example::Image'[part_def]))
      (action_def 'Shoot'
        (item_usage in 'image' : 'Merge Example::Image'[part_def])
        (item_usage out 'picture' : 'Merge Example::Picture'[part_def]))
      (action_def 'Display'
        (item_usage in 'picture' : 'Merge Example::Picture'[part_def]))
      (action_def 'TakePicture')
      (action_usage 'takePicture' : 'Merge Example::TakePicture'[action_def]
        (initial_node)
        (source_succession
          (merge_node 'continue'))
        (source_succession
          (action_usage 'trigger'
            (item_usage out 'scene' : 'Merge Example::Scene'[part_def])))
        (flow_usage composite
          (connector_end 'trigger.scene')
          (connector_end 'focus.scene'))
        (source_succession
          (action_usage 'focus' : 'Merge Example::Focus'[action_def]
            (item_usage in 'scene')
            (item_usage out 'image')))
        (flow_usage composite
          (connector_end 'focus.image')
          (connector_end 'shoot.image'))
        (source_succession
          (action_usage 'shoot' : 'Merge Example::Shoot'[action_def]
            (item_usage in 'image')
            (item_usage out 'picture')))
        (flow_usage composite
          (connector_end 'shoot.picture')
          (connector_end 'display.picture'))
        (source_succession
          (action_usage 'display' : 'Merge Example::Display'[action_def]
            (item_usage in 'picture')))
        (source_succession
          (reference_usage reference 'continue'))))))
~~~
