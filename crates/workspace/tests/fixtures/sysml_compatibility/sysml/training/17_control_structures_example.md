# META
~~~ini
description=SysML Training 17 (Control): Control Structures Example
type=file
~~~
# SOURCE
~~~sysml
package 'Control Structures Example' {
	private import ScalarValues::*;
	
	attribute def BatteryCharged;
	
	part battery;
	part powerSystem;
	
	action def MonitorBattery { out charge : Real; }
	action def AddCharge { in charge : Real; }
	action def EndCharging;
	
	action def ChargeBattery {
		loop action charging {
			action monitor : MonitorBattery {
				out charge;
			}
			
			then if monitor.charge < 100 {
				action addCharge : AddCharge {
					in charge = monitor.charge;
				}
			}				
		} until charging.monitor.charge >= 100;
		
		then action endCharging : EndCharging;
		then done;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,
KwLoop,KwAction,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwOut,Ident,Semicolon,
CloseCurly,
KwThen,KwIf,Ident,Dot,Ident,OpenAngle,DecimalValue,OpenCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,KwUntil,Ident,Dot,Ident,Dot,Ident,GtEq,DecimalValue,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,Semicolon,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Control Structures Example''
    (import_decl private 'ScalarValues::*')
    (attribute_def 'BatteryCharged')
    (part_usage 'battery')
    (part_usage 'powerSystem')
    (action_def 'MonitorBattery'
      (default_ref_usage out 'charge' : 'Real'))
    (action_def 'AddCharge'
      (default_ref_usage in 'charge' : 'Real'))
    (action_def 'EndCharging')
    (action_def 'ChargeBattery'
      (while_loop_node)
      (action_usage 'charging'
        (action_usage 'monitor' : 'MonitorBattery'
          (default_ref_usage out 'charge'))
        (source_succession
          (if_node)))
      (malformed)
      (source_succession
        (action_usage 'endCharging' : 'EndCharging'))
      (source_succession
        (default_ref_usage 'done')))))
~~~
# FORMAT
~~~sysml
package 'Control Structures Example' {
    private import ScalarValues::*;

    attribute def BatteryCharged;

    part battery;
    part powerSystem;

    action def MonitorBattery { out charge : Real; }
    action def AddCharge { in charge : Real; }
    action def EndCharging;

    action def ChargeBattery {
        loop action charging {
            action monitor : MonitorBattery {
                out charge;
            }

            then if monitor.charge < 100 {
                action addCharge : AddCharge {
                    in charge = monitor.charge;
                }
            }
        } until charging.monitor.charge >= 100;

        then action endCharging : EndCharging;
        then done;
    }
}

~~~
# EXPECTED
~~~
parse.unexpected_token
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
parse.unexpected_token
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Control Structures Example"))) (name "Control Structures Example") (declared-name "Control Structures Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Control Structures Example::*"))) (name "*") (declared-name "*"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Control Structures Example::AddCharge"))) (name "AddCharge") (declared-name "AddCharge")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Control Structures Example::AddCharge::charge"))) (name "charge") (declared-name "charge") (effective (featuring-type (node (document "d0") (qualified-name "Control Structures Example::AddCharge")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Control Structures Example::BatteryCharged"))) (name "BatteryCharged") (declared-name "BatteryCharged") (declared (properties (ordered false) (unique true))))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (name "ChargeBattery") (declared-name "ChargeBattery")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (name "endCharging") (declared-name "endCharging") (effective (featuring-type (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Control Structures Example::EndCharging"))) (name "EndCharging") (declared-name "EndCharging"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery"))) (name "MonitorBattery") (declared-name "MonitorBattery")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (name "charge") (declared-name "charge") (effective (featuring-type (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Control Structures Example::battery"))) (name "battery") (declared-name "battery") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Control Structures Example::powerSystem"))) (name "powerSystem") (declared-name "powerSystem") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (to (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (to (node (document "d0") (qualified-name "Control Structures Example::EndCharging"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "Control Structures Example::ChargeBattery::endCharging") (target-qualified "Control Structures Example::ChargeBattery::done"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/17_control_structures_example.md"
    (diagnostics
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
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 18))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 13 2) (end 13 208))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 23 4) (end 23 47))
      )
    )
  )
)
~~~
