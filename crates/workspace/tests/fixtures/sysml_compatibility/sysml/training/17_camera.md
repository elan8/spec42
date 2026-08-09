# META
~~~ini
description=SysML Training 17 (Control): Camera
type=file
~~~
# SOURCE
~~~sysml
package Camera {
	private import 'Action Decomposition'::*;
	
	part def Camera;
	part def FocusingSubsystem;
	part def ImagingSubsystem;
	
	part camera : Camera {
		ref item scene : Scene;
		part photos : Picture[*];
				
		part autoFocus {
			in ref item scene : Scene = camera::scene;		
			out ref item realImage : Image;
		}
		
		flow autoFocus.realImage to imager.focusedImage;
		
		part imager {
			in item focusedImage : Image;		
			out item photo : Picture :> photos;
		}
		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwRef,KwItem,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenCurly,
KwIn,KwRef,KwItem,Ident,Colon,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwOut,KwRef,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Camera'
    (import_decl private ''Action Decomposition'::*')
    (part_def 'Camera')
    (part_def 'FocusingSubsystem')
    (part_def 'ImagingSubsystem')
    (part_usage 'camera' : 'Camera'
      (item_usage ref 'scene' : 'Scene')
      (part_usage 'photos' : 'Picture' multiplicity)
      (part_usage 'autoFocus'
        (item_usage in ref 'scene' : 'Scene' value)
        (item_usage out ref 'realImage' : 'Image'))
      (flow_usage 'autoFocus')
      (part_usage 'imager'
        (item_usage in 'focusedImage' : 'Image')
        (item_usage out 'photo' : 'Picture' :> 'photos')))))
~~~
# FORMAT
~~~sysml
package Camera {
    private import 'Action Decomposition'::*;

    part def Camera;
    part def FocusingSubsystem;
    part def ImagingSubsystem;

    part camera : Camera {
        ref item scene : Scene;
        part photos : Picture [*];

        part autoFocus {
            in ref item scene : Scene = camera::scene;
            out ref item realImage : Image;
        }

        flow autoFocus;

        part imager {
            in item focusedImage : Image;
            out item photo : Picture :> photos;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'autoFocus'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Picture'
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Picture'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'autoFocus'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Picture'
semantic.unresolved_name 'Scene'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Image'
semantic.unresolved_name 'Picture'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Camera'
      (namespace_import private -> 'Action Decomposition'[unresolved])
      (part_def 'Camera')
      (part_def 'FocusingSubsystem')
      (part_def 'ImagingSubsystem')
      (part_usage 'camera' : 'Camera::Camera'[part_def]
        (item_usage reference 'scene' : 'Scene'[unresolved])
        (part_usage composite 'photos' : 'Picture'[unresolved]
          (multiplicity_range [*]))
        (part_usage composite 'autoFocus'
          (item_usage in reference 'scene' : 'Scene'[unresolved]
            (feature_value (=)))
          (item_usage out reference 'realImage' : 'Image'[unresolved]))
        (flow_usage composite 'autoFocus')
        (part_usage composite 'imager'
          (item_usage in 'focusedImage' : 'Image'[unresolved])
          (item_usage out 'photo' : 'Picture'[unresolved] :> 'Camera::camera::photos'[part_usage]))))))
~~~
