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

    action def MonitorBattery {
        out charge : Real;
    }
    action def AddCharge {
        in charge : Real;
    }
    action def EndCharging;

    action def ChargeBattery {
        first start;

        then merge continueCharging;

        then action monitor : MonitorBattery {
			out batteryCharge : Real;
		}

        then decide;
        if monitor.batteryCharge < 100;
        then addCharge;
        if monitor.batteryCharge >= 100;
        then endCharging;

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
(model
  (namespace
    (package 'Decision Example'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (attribute_def 'BatteryCharged')
      (part_usage 'battery')
      (part_usage 'powerSystem')
      (action_def 'MonitorBattery'
        (reference_usage out reference 'charge' : 'Real'[unresolved]))
      (action_def 'AddCharge'
        (reference_usage in reference 'charge' : 'Real'[unresolved]))
      (action_def 'EndCharging')
      (action_def 'ChargeBattery'
        (initial_node)
        (source_succession
          (merge_node 'continueCharging'))
        (source_succession
          (action_usage 'monitor' : 'Decision Example::MonitorBattery'[action_def]
            (reference_usage out reference 'batteryCharge' : 'Real'[unresolved])))
        (source_succession
          (decide_node))
        (if_action_usage)
        (source_succession
          (reference_usage reference 'addCharge'))
        (if_action_usage)
        (source_succession
          (reference_usage reference 'endCharging'))
        (action_usage composite 'addCharge' : 'Decision Example::AddCharge'[action_def]
          (reference_usage in reference 'charge'
            (feature_value (=))))
        (source_succession
          (reference_usage reference 'continueCharging'))
        (action_usage composite 'endCharging' : 'Decision Example::EndCharging'[action_def])
        (source_succession
          (reference_usage reference 'done'))))))
~~~
