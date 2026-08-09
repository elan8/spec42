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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwFork,Semicolon,
KwThen,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwPerform,Ident,Semicolon,
KwPerform,Ident,Semicolon,
KwThen,KwTerminate,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwAction,Ident,KwTerminate,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Terminate Actions Example-1''
    (import_decl private 'ScalarValues::Boolean')
    (action_usage 'monitorCriticalActivity')
    (action_usage 'criticalActivity')
    (action_usage 'waitForTimeOut')
    (action_def 'MonitoredActivity'
      (initial_node start)
      (source_succession
        (sysml_decl))
      (source_succession
        (default_ref_usage 'performCriticalActivity'))
      (source_succession
        (default_ref_usage 'waitForTimeOut'))
      (action_usage 'performCriticalActivity'
        (perform_action :>> 'monitorCriticalActivity')
        (perform_action :>> 'criticalActivity')
        (source_succession
          (terminate_node)))
      (source_succession
        (default_ref_usage 'stop'))
      (action_usage 'waitForTimeOut')
      (source_succession
        (default_ref_usage 'stop'))
      (action_usage 'stop')
      (terminate_node))))
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
# EXPECTED
~~~
semantic.duplicate_name 'performCriticalActivity'
semantic.duplicate_name 'waitForTimeOut'
semantic.duplicate_name 'stop'
semantic.duplicate_name 'stop'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'performCriticalActivity'
semantic.duplicate_name 'waitForTimeOut'
semantic.duplicate_name 'stop'
semantic.duplicate_name 'stop'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Terminate Actions Example-1"))) (name "Terminate Actions Example-1") (declared-name "Terminate Actions Example-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (name "MonitoredActivity") (declared-name "MonitoredActivity")
          (contains
            (element (kind "initial") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::_initial"))) (name "_initial") (effective (featuring-type (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))))
            (element (kind "terminate") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::_terminate"))) (name "terminate") (declared-name "terminate") (effective (featuring-type (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (name "performCriticalActivity") (declared-name "performCriticalActivity") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (name "stop") (declared-name "stop") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (name "waitForTimeOut") (declared-name "waitForTimeOut") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::criticalActivity"))) (name "criticalActivity") (declared-name "criticalActivity") (declared))
        (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::monitorCriticalActivity"))) (name "monitorCriticalActivity") (declared-name "monitorCriticalActivity") (declared))
        (element (kind "action") (id (node (document "d0") (qualified-name "Terminate Actions Example-1::waitForTimeOut"))) (name "waitForTimeOut") (declared-name "waitForTimeOut") (declared))
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (provenance authored))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (provenance authored))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (to (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (provenance authored))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "Terminate Actions Example-1::MonitoredActivity") (target-qualified "Terminate Actions Example-1::MonitoredActivity::fork"))
    (flow (status pending) (document "d0") (source-qualified "Terminate Actions Example-1::MonitoredActivity::_initial") (target-qualified "Terminate Actions Example-1::MonitoredActivity::start"))
    (flow (status pending) (document "d0") (source-qualified "Terminate Actions Example-1::MonitoredActivity::fork") (target-qualified "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))
    (flow (status pending) (document "d0") (source-qualified "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity") (target-qualified "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity::terminate"))
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::_terminate"))) (status missing-prerequisite) (target "Actions::terminateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::performCriticalActivity"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::stop"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::MonitoredActivity::waitForTimeOut"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::criticalActivity"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::monitorCriticalActivity"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Terminate Actions Example-1::waitForTimeOut"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/19_terminate_actions_example_1.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
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
