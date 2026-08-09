# META
~~~ini
description=SysML Example (Camera): Camera
type=file
~~~
# SOURCE
~~~sysml
part def Camera {
	private import PictureTaking::*;
	
	perform action takePicture[*] :> PictureTaking::takePicture;
	
	part focusingSubsystem {
		perform takePicture.focus;
	}
	
	part imagingSubsystem {
		perform takePicture.shoot;
	}
	
}
~~~
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPerform,KwAction,Ident,OpenSquare,Star,CloseSquare,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Camera'
    (import_decl private 'PictureTaking::*')
    (perform_action 'takePicture' :> 'PictureTaking::takePicture' multiplicity)
    (part_usage 'focusingSubsystem'
      (perform_action :>> 'takePicture.focus'))
    (part_usage 'imagingSubsystem'
      (perform_action :>> 'takePicture.shoot'))))
~~~
# FORMAT
~~~sysml
part def Camera {
    private import PictureTaking::*;

    perform action takePicture :> PictureTaking::takePicture [*];

    part focusingSubsystem {
        perform :>> takePicture.focus;
    }

    part imagingSubsystem {
        perform :>> takePicture.shoot;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'PictureTaking::takePicture'
semantic.unresolved_name 'takePicture::focus'
semantic.unresolved_name 'takePicture::shoot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'PictureTaking::takePicture'
semantic.unresolved_name 'takePicture::focus'
semantic.unresolved_name 'takePicture::shoot'
~~~
# SMG
~~~
(model
  (namespace
    (part_def 'Camera'
      (namespace_import private -> 'PictureTaking'[unresolved])
      (perform_action_usage 'takePicture' :> 'PictureTaking::takePicture'[unresolved]
        (multiplicity_range [*]))
      (part_usage composite 'focusingSubsystem'
        (perform_action_usage :>> 'takePicture::focus'[unresolved]))
      (part_usage composite 'imagingSubsystem'
        (perform_action_usage :>> 'takePicture::shoot'[unresolved])))))
~~~
