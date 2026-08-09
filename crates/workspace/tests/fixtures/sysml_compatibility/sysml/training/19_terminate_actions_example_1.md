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
            perform :>> monitorCriticalActivity;

            perform :>> criticalActivity;
            then terminate;
        }
        then stop;

        action waitForTimeOut;
        then stop;

        action stop;
        terminate;
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
(model
  (namespace
    (package 'Terminate Actions Example-1'
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (action_usage 'monitorCriticalActivity')
      (action_usage 'criticalActivity')
      (action_usage 'waitForTimeOut')
      (action_def 'MonitoredActivity'
        (initial_node)
        (source_succession
          (fork_node))
        (source_succession
          (reference_usage reference 'performCriticalActivity'))
        (source_succession
          (reference_usage reference 'waitForTimeOut'))
        (action_usage composite 'performCriticalActivity'
          (perform_action_usage :>> 'Terminate Actions Example-1::monitorCriticalActivity'[action_usage])
          (perform_action_usage :>> 'Terminate Actions Example-1::criticalActivity'[action_usage])
          (source_succession
            (terminate_action_usage)))
        (source_succession
          (reference_usage reference 'stop'))
        (action_usage composite 'waitForTimeOut')
        (source_succession
          (reference_usage reference 'stop'))
        (action_usage composite 'stop')
        (terminate_action_usage)))))
~~~
