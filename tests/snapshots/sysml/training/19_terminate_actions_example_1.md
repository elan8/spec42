# META
~~~ini
description=SysML Training 19 (Terminate Actions): Terminate Actions Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Terminate Actions Example-1' {
	private import ScalarValues::Boolean;
	
	action monitorCriticalActivity;
	action criticalActivity;
	action waitForTimeOut;
	
	action def MonitoredActivity {
		first start;

		then fork;
			then performCriticalActivity;
			then waitForTimeOut;
					
		action performCriticalActivity {
			perform monitorCriticalActivity;
			
			perform criticalActivity;
			then terminate;
		}
		then stop;
		
		action waitForTimeOut;
		then stop;
				
		action stop terminate;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/19_terminate_actions_example_1.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 8) (end 8 13))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 15 3) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 15 3) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 8) (end 18 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3555b074c24e17f1d6248fbb966bdf0673d190b526a2b3beb6aa84569bd88528") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind fork) (ordinal 0))))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "performCriticalActivity")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "waitForTimeOut")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 2))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "stop")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 3))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "stop")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminate-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (named (kind action) (name "performCriticalActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "terminate")))))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::criticalActivity"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::monitorCriticalActivity"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::waitForTimeOut"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "performCriticalActivity")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity")))))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "waitForTimeOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut")))))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0))
      (authored-target "stop")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop")))))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0))
      (authored-target "stop")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop")))))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (named (kind action) (name "performCriticalActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "terminate")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind fork) (ordinal 0))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (named (kind action) (name "performCriticalActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind fork) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind terminate-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity")))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (named (kind action) (name "performCriticalActivity")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop")))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
    (declaration (id (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut")))
      (featured-by (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/19_terminate_actions_example_1.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_1.md") (range (start 8 8) (end 8 13)) (probe (position 8 8))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_1.md") (range (start 11 8) (end 11 31)) (probe (position 11 8))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "performCriticalActivity")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity")))))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_1.md") (range (start 12 8) (end 12 22)) (probe (position 12 8))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "waitForTimeOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut")))))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_1.md") (range (start 20 7) (end 20 11)) (probe (position 20 7))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0) (authored-target "stop")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop")))))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_1.md") (range (start 23 7) (end 23 11)) (probe (position 23 7))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0) (authored-target "stop")
      (outcome (status resolved) (target (node (document "memory://snapshot/19_terminate_actions_example_1.md") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop")))))
    )
  )
  (query (document "memory://snapshot/19_terminate_actions_example_1.md") (range (start 18 8) (end 18 17)) (probe (position 18 8))
    (reference (id (source (node (document "memory://snapshot/19_terminate_actions_example_1.md") (path (named (kind package) (name "Terminate Actions Example-1")) (named (kind action-def) (name "MonitoredActivity")) (named (kind action) (name "performCriticalActivity")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "terminate")
      (outcome (status unresolved)))
    )
  )
)
~~~
