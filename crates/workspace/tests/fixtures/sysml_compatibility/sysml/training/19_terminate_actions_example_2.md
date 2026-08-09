# META
~~~ini
description=SysML Training 19 (Terminate Actions): Terminate Actions Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Terminate Actions Example-2' {
	action def WorkflowProcess;
	
	part def Processor {
		ref action workflowProcess : WorkflowProcess;
		
		action internalProcess {
			// ...
		}
	}
		
	action terminateProcessing {
		in processor : Processor;
		
		terminate processor.workflowProcess;
				
		terminate processor;
	}

}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAction,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwRef,KwAction,Ident,Colon,Ident,Semicolon,
KwAction,Ident,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwTerminate,Ident,Dot,Ident,Semicolon,
KwTerminate,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Terminate Actions Example-2''
    (action_def 'WorkflowProcess')
    (part_def 'Processor'
      (action_usage ref 'workflowProcess' : 'WorkflowProcess')
      (action_usage 'internalProcess'
        (line_comment)))
    (action_usage 'terminateProcessing'
      (default_ref_usage in 'processor' : 'Processor')
      (terminate_node processor.workflowProcess)
      (terminate_node processor))))
~~~
# FORMAT
~~~sysml
package 'Terminate Actions Example-2' {
    action def WorkflowProcess;

    part def Processor {
        ref action workflowProcess : WorkflowProcess;

        action internalProcess {
            // ...
        }
    }

    action terminateProcessing {
        in processor : Processor;

        terminate processor.workflowProcess;

        terminate processor;
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
    (package 'Terminate Actions Example-2'
      (action_def 'WorkflowProcess')
      (part_def 'Processor'
        (action_usage reference 'workflowProcess' : 'Terminate Actions Example-2::WorkflowProcess'[action_def])
        (action_usage composite 'internalProcess'))
      (action_usage 'terminateProcessing'
        (reference_usage in reference 'processor' : 'Terminate Actions Example-2::Processor'[part_def])
        (terminate_action_usage)
        (terminate_action_usage)))))
~~~
