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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "19_terminate_actions_example_2.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "02c0d8f746a16f59fad762ee612f0c8c34c1f9c8cb5454e61e1b4b2f42abe52b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2"))) (kind "package") (name "Terminate Actions Example-2") (declared-name "Terminate Actions Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 324))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (kind "part def") (name "Processor") (declared-name "Processor") (range (start (line 3) (character 1)) (end (line 3) (character 116))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::internalProcess"))) (kind "action") (name "internalProcess") (declared-name "internalProcess") (range (start (line 6) (character 2)) (end (line 6) (character 40))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind "action") (name "workflowProcess") (declared-name "workflowProcess") (range (start (line 4) (character 2)) (end (line 4) (character 47))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (authored (membership (kind Feature)) (relationships (typing (reference "WorkflowProcess") (range none)))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (kind "action def") (name "WorkflowProcess") (declared-name "WorkflowProcess") (range (start (line 1) (character 1)) (end (line 1) (character 28))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind "action") (name "terminateProcessing") (declared-name "terminateProcessing") (range (start (line 11) (character 1)) (end (line 11) (character 130))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate"))) (kind "terminate") (name "terminate") (declared-name "terminate") (range (start (line 14) (character 2)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate#terminate"))) (kind "terminate") (name "terminate") (declared-name "terminate") (range (start (line 16) (character 2)) (end (line 16) (character 22))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind "in out parameter") (name "processor") (declared-name "processor") (range (start (line 12) (character 2)) (end (line 12) (character 27))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (authored (relationships (typing (reference "Processor") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0)) (authored-target "WorkflowProcess") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess")))))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind featureTyping) (ordinal 0)) (authored-target "Processor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
