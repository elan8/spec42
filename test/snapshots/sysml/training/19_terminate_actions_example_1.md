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
  (document "19_terminate_actions_example_1.md"
    (diagnostics
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
        (range (start 7 1) (end 7 337))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 2) (end 8 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 2) (end 14 126))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 15 3) (end 15 43))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 15 3) (end 15 43))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "bf82b9367aff349618fe9c8bf6a9ce8e7744b40dcc9df41ca35d9a76ad7a7612") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1"))) (kind "package") (name "Terminate Actions Example-1") (declared-name "Terminate Actions Example-1") (range (start (line 0) (character 0)) (end (line 0) (character 505))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind "action def") (name "MonitoredActivity") (declared-name "MonitoredActivity") (range (start (line 7) (character 1)) (end (line 7) (character 337))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1"))) (authored (membership (kind Owning)) (relationships (flow (reference "Terminate Actions Example-1::MonitoredActivity::fork") (range none)) (perform (reference "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity") (range none)) (perform (reference "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut") (range none)) (perform (reference "Terminate Actions Example-1::MonitoredActivity::stop") (range none)))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::_initial"))) (kind "initial") (name "_initial") (range (start (line 8) (character 2)) (end (line 8) (character 14))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (authored (relationships (flow (reference "Terminate Actions Example-1::MonitoredActivity::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::_terminate"))) (kind "terminate") (name "terminate") (declared-name "terminate") (range (start (line 25) (character 14)) (end (line 25) (character 24))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (kind "action") (name "performCriticalActivity") (declared-name "performCriticalActivity") (range (start (line 14) (character 2)) (end (line 14) (character 126))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (authored (membership (kind Feature)) (relationships (flow (reference "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity::terminate") (range none)))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (kind "action") (name "stop") (declared-name "stop") (range (start (line 25) (character 2)) (end (line 25) (character 14))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (kind "action") (name "waitForTimeOut") (declared-name "waitForTimeOut") (range (start (line 22) (character 2)) (end (line 22) (character 24))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::criticalActivity"))) (kind "action") (name "criticalActivity") (declared-name "criticalActivity") (range (start (line 4) (character 1)) (end (line 4) (character 25))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::monitorCriticalActivity"))) (kind "action") (name "monitorCriticalActivity") (declared-name "monitorCriticalActivity") (range (start (line 3) (character 1)) (end (line 3) (character 32))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1"))))
    (element (id (node (document "d0") (qualified-name "Terminate Actions Example-1::waitForTimeOut"))) (kind "action") (name "waitForTimeOut") (declared-name "waitForTimeOut") (range (start (line 5) (character 1)) (end (line 5) (character 23))) (parent (node (document "d0") (qualified-name "Terminate Actions Example-1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-1::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 1) (character 16)) (end (line 1) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind flowSource) (ordinal 0)) (authored-target "Terminate Actions Example-1::MonitoredActivity::fork") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind performSource) (ordinal 0)) (authored-target "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity")))))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind performSource) (ordinal 1)) (authored-target "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut")))))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind performSource) (ordinal 2)) (authored-target "Terminate Actions Example-1::MonitoredActivity::stop") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop")))))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "Terminate Actions Example-1::MonitoredActivity::start") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (kind flowSource) (ordinal 0)) (authored-target "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity::terminate") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (target (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (target (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (target (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (kind performSource) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
