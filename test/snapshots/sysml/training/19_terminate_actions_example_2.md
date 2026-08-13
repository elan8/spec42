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
  (document "memory://snapshot/19_terminate_actions_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 12 2) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 14 2) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 16 2) (end 16 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:7a5095e259d46b27a63a257e3aa3bbe06f5e139ff8dced52719d91f7e581b968") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::internalProcess"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WorkflowProcess"))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0))
      (authored-target "WorkflowProcess")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/19_terminate_actions_example_2.md") (range (start 4 31) (end 4 46)) (probe (position 4 31))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0) (authored-target "WorkflowProcess")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess")))))
  )
)
~~~
