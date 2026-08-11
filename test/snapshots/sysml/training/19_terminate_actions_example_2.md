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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Terminate Actions Example-2"))) (name "Terminate Actions Example-2") (declared-name "Terminate Actions Example-2")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (name "Processor") (declared-name "Processor") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::internalProcess"))) (name "internalProcess") (declared-name "internalProcess") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (name "workflowProcess") (declared-name "workflowProcess") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (name "WorkflowProcess") (declared-name "WorkflowProcess"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (name "terminateProcessing") (declared-name "terminateProcessing") (declared)
          (contains
            (element (kind "terminate") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate"))) (name "terminate") (declared-name "terminate"))
            (element (kind "terminate") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate#terminate"))) (name "terminate") (declared-name "terminate"))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (name "processor") (declared-name "processor") (declared (properties (direction "in"))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::internalProcess"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate"))) (status missing-prerequisite) (target "Actions::terminateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate#terminate"))) (status missing-prerequisite) (target "Actions::terminateActions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/19_terminate_actions_example_2.md"
    (diagnostics
    )
  )
)
~~~
