# META
~~~ini
description=SysML Training 17 (Control): Decision Example
type=file
~~~
# SOURCE
~~~sysml
package 'Decision Example' {
	private import ScalarValues::*;
	
	attribute def BatteryCharged;
	
	part battery;
	part powerSystem;
	
	action def MonitorBattery { out charge : Real; }
	action def AddCharge { in charge : Real; }
	action def EndCharging;
	
	action def ChargeBattery {
		first start;

		then merge continueCharging;
		
		then action monitor : MonitorBattery {
			out batteryCharge : Real;
		}
		
		then decide;
			if monitor.batteryCharge < 100 then addCharge;
			if monitor.batteryCharge >= 100 then endCharging;
			
		action addCharge : AddCharge {
			in charge = monitor.batteryCharge;
		}
		then continueCharging;
		
		action endCharging : EndCharging;
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
KwFirst,Ident,Semicolon,
KwThen,KwMerge,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwThen,KwDecide,Semicolon,
KwIf,Ident,Dot,Ident,OpenAngle,DecimalValue,KwThen,Ident,Semicolon,
KwIf,Ident,Dot,Ident,GtEq,DecimalValue,KwThen,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwAction,Ident,Colon,Ident,Semicolon,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Decision Example''
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
      (initial_node start)
      (source_succession
        (sysml_decl 'continueCharging'))
      (source_succession
        (action_usage 'monitor' : 'MonitorBattery'
          (default_ref_usage out 'batteryCharge' : 'Real')))
      (source_succession
        (sysml_decl))
      (if_node)
      (source_succession
        (default_ref_usage 'addCharge'))
      (if_node)
      (source_succession
        (default_ref_usage 'endCharging'))
      (action_usage 'addCharge' : 'AddCharge'
        (default_ref_usage in 'charge' value))
      (source_succession
        (default_ref_usage 'continueCharging'))
      (action_usage 'endCharging' : 'EndCharging')
      (source_succession
        (default_ref_usage 'done')))))
~~~
# FORMAT
~~~sysml
package 'Decision Example' {
    private import ScalarValues::*;

    attribute def BatteryCharged;

    part battery;
    part powerSystem;

    action def MonitorBattery { out charge : Real; }
    action def AddCharge { in charge : Real; }
    action def EndCharging;

    action def ChargeBattery {
        first start;

        then merge continueCharging;

        then action monitor : MonitorBattery {
            out batteryCharge : Real;
        }

        then decide;
        if monitor.batteryCharge < 100 then addCharge;
        if monitor.batteryCharge >= 100 then endCharging;

        action addCharge : AddCharge {
            in charge = monitor.batteryCharge;
        }
        then continueCharging;

        action endCharging : EndCharging;
        then done;
    }
}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'addCharge'
semantic.duplicate_name 'continueCharging'
semantic.duplicate_name 'endCharging'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'addCharge'
semantic.duplicate_name 'continueCharging'
semantic.duplicate_name 'endCharging'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Decision Example"))) (name "Decision Example") (declared-name "Decision Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Decision Example::*"))) (name "*") (declared-name "*"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Decision Example::AddCharge"))) (name "AddCharge") (declared-name "AddCharge")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Decision Example::AddCharge::charge"))) (name "charge") (declared-name "charge") (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::AddCharge")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Decision Example::BatteryCharged"))) (name "BatteryCharged") (declared-name "BatteryCharged") (declared (properties (ordered false) (unique true))))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (name "ChargeBattery") (declared-name "ChargeBattery")
          (contains
            (element (kind "initial") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::_initial"))) (name "_initial") (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::ChargeBattery")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (name "addCharge") (declared-name "addCharge") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge::charge"))) (name "charge") (declared-name "charge") (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::AddCharge")))))
              )
            )
            (element (kind "merge") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::continueCharging"))) (name "merge") (declared-name "merge") (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::ChargeBattery")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (name "endCharging") (declared-name "endCharging") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::ChargeBattery")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (name "monitor") (declared-name "monitor") (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor::batteryCharge"))) (name "batteryCharge") (declared-name "batteryCharge") (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::MonitorBattery")))))
              )
            )
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Decision Example::EndCharging"))) (name "EndCharging") (declared-name "EndCharging"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Decision Example::MonitorBattery"))) (name "MonitorBattery") (declared-name "MonitorBattery")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Decision Example::MonitorBattery::charge"))) (name "charge") (declared-name "charge") (effective (featuring-type (node (document "d0") (qualified-name "Decision Example::MonitorBattery")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Decision Example::battery"))) (name "battery") (declared-name "battery") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Decision Example::powerSystem"))) (name "powerSystem") (declared-name "powerSystem") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "Decision Example::ChargeBattery::continueCharging"))) (to (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (to (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (to (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (to (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (to (node (document "d0") (qualified-name "Decision Example::AddCharge"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (to (node (document "d0") (qualified-name "Decision Example::EndCharging"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (to (node (document "d0") (qualified-name "Decision Example::MonitorBattery"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "Decision Example::ChargeBattery::_initial") (target-qualified "Decision Example::ChargeBattery::start"))
    (flow (status pending) (document "d0") (source-qualified "Decision Example::ChargeBattery::continueCharging") (target-qualified "Decision Example::ChargeBattery::done"))
    (flow (status pending) (document "d0") (source-qualified "Decision Example::ChargeBattery::decide") (target-qualified "Decision Example::ChargeBattery::continueCharging"))
    (flow (status pending) (document "d0") (source-qualified "Decision Example::ChargeBattery::monitor") (target-qualified "Decision Example::ChargeBattery::decide"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/17_decision_example.md"
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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 29) (end 8 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 24) (end 9 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 3) (end 18 28))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 22 3) (end 22 53))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 22 3) (end 22 53))
      )
    )
  )
)
~~~
