# META
~~~ini
description=SysML Training 18 (Action Performance): Action Performance Example
type=file
~~~
# SOURCE
~~~sysml
package 'Action Performance Example' {
	private import 'Action Decomposition'::*;
	
	part def Camera;
	part def AutoFocus;
	part def Imager;
	
	part camera : Camera {
		
		perform action takePhoto[*] ordered 
			references takePicture;
		
		part f : AutoFocus {
			perform takePhoto.focus;			
		}
		
		part i : Imager {
			perform takePhoto.shoot;
		}		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,KwAction,Ident,OpenSquare,Star,CloseSquare,KwOrdered,
KwReferences,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Performance Example''
    (import_decl private ''Action Decomposition'::*')
    (part_def 'Camera')
    (part_def 'AutoFocus')
    (part_def 'Imager')
    (part_usage 'camera' : 'Camera'
      (perform_action 'takePhoto' references 'takePicture' multiplicity ordered)
      (part_usage 'f' : 'AutoFocus'
        (perform_action :>> 'takePhoto.focus'))
      (part_usage 'i' : 'Imager'
        (perform_action :>> 'takePhoto.shoot')))))
~~~
# FORMAT
~~~sysml
package 'Action Performance Example' {
    private import 'Action Decomposition'::*;

    part def Camera;
    part def AutoFocus;
    part def Imager;

    part camera : Camera {
        perform action takePhoto references takePicture [*] ordered;

        part f : AutoFocus {
            perform :>> takePhoto.focus;
        }

        part i : Imager {
            perform :>> takePhoto.shoot;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'takePicture'
semantic.unresolved_name 'takePhoto::focus'
semantic.unresolved_name 'takePhoto::shoot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'takePicture'
semantic.unresolved_name 'takePhoto::focus'
semantic.unresolved_name 'takePhoto::shoot'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Action Performance Example'
      (namespace_import private -> 'Action Decomposition'[unresolved])
      (part_def 'Camera')
      (part_def 'AutoFocus')
      (part_def 'Imager')
      (part_usage 'camera' : 'Action Performance Example::Camera'[part_def]
        (perform_action_usage ordered 'takePhoto' :> 'takePicture'[unresolved]
          (multiplicity_range [*]))
        (part_usage composite 'f' : 'Action Performance Example::AutoFocus'[part_def]
          (perform_action_usage :>> 'takePhoto::focus'[unresolved]))
        (part_usage composite 'i' : 'Action Performance Example::Imager'[part_def]
          (perform_action_usage :>> 'takePhoto::shoot'[unresolved]))))))
~~~
