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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 12) (end 16 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7a5095e259d46b27a63a257e3aa3bbe06f5e139ff8dced52719d91f7e581b968") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::internalProcess"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WorkflowProcess")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "processor::workflowProcess")) (terminateTarget (reference "processor")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Processor") (direction in)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0))
      (authored-target "WorkflowProcess")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess")))))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "processor::workflowProcess")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess")))))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind terminateTarget) (ordinal 0))
      (authored-target "processor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")))
      (subtype (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::internalProcess")))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess")))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")))
      (type (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess")) (provenance authored))
      (effective-type (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess")) (source direct))
      (supertype (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::WorkflowProcess")))
      (subtype (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor")))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing")))
      (type (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")) (source direct))
      (supertype (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")) (scopes any))
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
  (query (document "memory://snapshot/19_terminate_actions_example_2.md") (range (start 14 12) (end 14 37)) (probe (position 14 12))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind memberAccessOperand) (ordinal 0) (authored-target "processor::workflowProcess")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor::workflowProcess")))))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_2.md") (range (start 16 12) (end 16 21)) (probe (position 16 12))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing"))) (kind terminateTarget) (ordinal 0) (authored-target "processor")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_2.md") (range (start 12 17) (end 12 26)) (probe (position 12 17))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::terminateProcessing::processor"))) (kind featureTyping) (ordinal 0) (authored-target "Processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_2.md") (qualified-name "Terminate Actions Example-2::Processor")))))
    )
  )
)
~~~
