# META
~~~ini
description=KerML Behavior: TakePicture
type=file
~~~
# SOURCE
~~~kerml
behavior TakePicture {
	private import Camera;
	
	feature camera: Camera[1] subsets involvedObjects;
	
	class Exposure;
	
	behavior Focus { out xrsl: Exposure; }
	behavior Shoot { in xsf: Exposure; }
	
	step step1: Focus[1];	
	step step2: Shoot[1];
	
	succession flow exposure[1] of Exposure from step1.xrsl to step2.xsf;

	succession step1 then camera.focusedState;
	succession step2 then camera.shotState;
}
~~~
# TOKENS
~~~zig
KwBehavior,Ident,OpenCurly,
KwPrivate,KwImport,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwBehavior,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwBehavior,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwFlow,Ident,OpenSquare,DecimalValue,CloseSquare,KwOf,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwSuccession,Ident,KwThen,Ident,Dot,Ident,Semicolon,
KwSuccession,Ident,KwThen,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (behavior_def
    (import_decl private 'Camera')
    (feature_def 'camera' : 'Camera' multiplicity :> 'involvedObjects')
    (class_def 'Exposure')
    (behavior_def
      (feature_def out 'xrsl' : 'Exposure'))
    (behavior_def
      (feature_def in 'xsf' : 'Exposure'))
    (step_def)
    (step_def)
    (succession_flow_feature 'exposure' : 'Exposure' multiplicity
      (connector_end)
      (connector_end))
    (succession_def
      (connector_end)
      (connector_end))
    (succession_def
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
behavior TakePicture {
    private import Camera;

    feature camera : Camera [1] subsets involvedObjects;

    class Exposure;

    behavior Focus {
        out xrsl: Exposure;
    }
    behavior Shoot {
        in xsf: Exposure;
    }

    step step1: Focus[1];
    step step2: Shoot[1];

    succession flow exposure [1] of Exposure from step1.xrsl to step2.xsf;

    succession step1 then camera.focusedState;
    succession step2 then camera.shotState;
}
~~~
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Camera'
semantic.unresolved_name 'involvedObjects'
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Camera'
semantic.unresolved_name 'involvedObjects'
~~~
# SMG
~~~
(model
  (namespace
    (behavior_def 'TakePicture'
      (membership_import private -> 'Camera'[unresolved])
      (feature_def 'camera' : 'Camera'[unresolved] :> 'involvedObjects'[unresolved]
        (multiplicity_range [1]))
      (class_def 'Exposure')
      (behavior_def 'Focus'
        (feature_def out 'xrsl' : 'TakePicture::Exposure'[class_def]))
      (behavior_def 'Shoot'
        (feature_def in 'xsf' : 'TakePicture::Exposure'[class_def]))
      (step_def 'step1' : 'TakePicture::Focus'[behavior_def]
        (multiplicity_range [1]))
      (step_def 'step2' : 'TakePicture::Shoot'[behavior_def]
        (multiplicity_range [1]))
      (flow_usage composite 'exposure' : 'TakePicture::Exposure'[class_def]
        (multiplicity_range [1])
        (connector_end 'step1.xrsl')
        (connector_end 'step2.xsf'))
      (succession_def
        (connector_end 'step1')
        (connector_end 'camera.focusedState'))
      (succession_def
        (connector_end 'step2')
        (connector_end 'camera.shotState')))))
~~~
