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

    action def MonitorBattery {
        out charge : Real;
    }
    action def AddCharge {
        in charge : Real;
    }
    action def EndCharging;

    action def ChargeBattery {
        loop { }
        action charging {
            action monitor : MonitorBattery {
                out charge;
            }

            then if monitor.charge < 100 {
				action addCharge : AddCharge {
					in charge = monitor.charge;
				}
			}
        }
        until charging.monitor.charge >= 100;

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
(model
  (namespace
    (package 'Control Structures Example'
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
        (while_loop_action_usage)
        (action_usage composite 'charging'
          (action_usage composite 'monitor' : 'Control Structures Example::MonitorBattery'[action_def]
            (reference_usage out reference 'charge'))
          (source_succession
            (if_action_usage
              (action_usage 'addCharge' : 'Control Structures Example::AddCharge'[action_def]
                (reference_usage in reference 'charge'
                  (feature_value (=)))))))
        (not_implemented 'malformed')
        (source_succession
          (action_usage 'endCharging' : 'Control Structures Example::EndCharging'[action_def]))
        (source_succession
          (reference_usage reference 'done'))))))
~~~
