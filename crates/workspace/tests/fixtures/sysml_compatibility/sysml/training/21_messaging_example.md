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

    action def Focus {
        in item scene : Scene;
        out item image : Image;
    }
    action def Shoot {
        in item image : Image;
        out item picture : Picture;
    }
    action def TakePicture;

    action screen;

    action takePicture : TakePicture {
        action trigger;
        accept scene : Scene;

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
(model
  (namespace
    (package 'Messaging Example'
      (item_def 'Scene')
      (item_def 'Image')
      (item_def 'Picture')
      (attribute_def 'Show'
        (item_usage composite 'picture' : 'Messaging Example::Picture'[item_def]))
      (action_def 'Focus'
        (item_usage in 'scene' : 'Messaging Example::Scene'[item_def])
        (item_usage out 'image' : 'Messaging Example::Image'[item_def]))
      (action_def 'Shoot'
        (item_usage in 'image' : 'Messaging Example::Image'[item_def])
        (item_usage out 'picture' : 'Messaging Example::Picture'[item_def]))
      (action_def 'TakePicture')
      (action_usage 'screen')
      (action_usage 'takePicture' : 'Messaging Example::TakePicture'[action_def]
        (action_usage composite 'trigger')
        (accept_action_usage)
        (source_succession
          (action_usage 'focus' : 'Messaging Example::Focus'[action_def]
            (item_usage in 'scene'
              (feature_value (=)))
            (item_usage out 'image')))
        (flow_usage composite
          (connector_end 'focus.image')
          (connector_end 'shoot.image'))
        (source_succession
          (action_usage 'shoot' : 'Messaging Example::Shoot'[action_def]
            (item_usage in 'image')
            (item_usage out 'picture')))
        (source_succession
          (send_action_usage))))))
~~~
