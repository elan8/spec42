# META
~~~ini
description=KerML Behavior: TakePicture
type=file
semantic_graph=skip
semantic_graph_skip_reason=strictly parsed non-empty source produced no typed semantic graph facts
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
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
