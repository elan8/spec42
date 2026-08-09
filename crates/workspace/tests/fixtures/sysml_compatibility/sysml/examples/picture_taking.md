# META
~~~ini
description=SysML Example (Camera): PictureTaking
type=file
~~~
# SOURCE
~~~sysml
package PictureTaking {
	part def Exposure;
	
	action def Focus { out xrsl: Exposure; }
	action def Shoot { in xsf: Exposure; }	
		
	action takePicture {		
		action focus: Focus[1];
		flow of Exposure from focus.xrsl to shoot.xsf;
		action shoot: Shoot[1];
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFlow,KwOf,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'PictureTaking'
    (part_def 'Exposure')
    (action_def 'Focus'
      (default_ref_usage out 'xrsl' : 'Exposure'))
    (action_def 'Shoot'
      (default_ref_usage in 'xsf' : 'Exposure'))
    (action_usage 'takePicture'
      (action_usage 'focus' : 'Focus' multiplicity)
      (flow_usage 'of')
      (action_usage 'shoot' : 'Shoot' multiplicity))))
~~~
# FORMAT
~~~sysml
package PictureTaking {
    part def Exposure;

    action def Focus {
        out xrsl : Exposure;
    }
    action def Shoot {
        in xsf : Exposure;
    }

    action takePicture {
        action focus : Focus [1];
        flow of;
        action shoot : Shoot [1];
    }
}
~~~
# EXPECTED
~~~
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(model
  (namespace
    (package 'PictureTaking'
      (part_def 'Exposure')
      (action_def 'Focus'
        (reference_usage out reference 'xrsl' : 'PictureTaking::Exposure'[part_def]))
      (action_def 'Shoot'
        (reference_usage in reference 'xsf' : 'PictureTaking::Exposure'[part_def]))
      (action_usage 'takePicture'
        (action_usage composite 'focus' : 'PictureTaking::Focus'[action_def]
          (multiplicity_range [1]))
        (flow_usage composite 'of')
        (action_usage composite 'shoot' : 'PictureTaking::Shoot'[action_def]
          (multiplicity_range [1]))))))
~~~
