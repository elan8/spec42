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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5158bfd670672b6a790faa5d2576494c77d8dc0073f2d67b67438b321e9b40bf") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2"))) (kind "package") (name "Terminate Actions Example-2") (declared-name "Terminate Actions Example-2"))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (kind "part def") (name "Processor") (declared-name "Processor") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::internalProcess"))) (kind "action") (name "internalProcess") (declared-name "internalProcess") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind "action") (name "workflowProcess") (declared-name "workflowProcess") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (authored (membership (kind Feature)) (relationships (typing (reference "WorkflowProcess")))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (kind "action def") (name "WorkflowProcess") (declared-name "WorkflowProcess") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind "action") (name "terminateProcessing") (declared-name "terminateProcessing") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate"))) (kind "terminate") (name "terminate") (declared-name "terminate") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::_terminate#terminate"))) (kind "terminate") (name "terminate") (declared-name "terminate") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind "in out parameter") (name "processor") (declared-name "processor") (parent (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (authored (relationships (typing (reference "Processor")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0)) (authored-target "WorkflowProcess") (outcome (status resolved) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess")))))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind featureTyping) (ordinal 0)) (authored-target "Processor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (target (node (document "d0") (qualified-name "Terminate Actions Example-2::Processor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
