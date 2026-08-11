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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_control_structures_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
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
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 2) (end 25 40))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1d29daa0f52c5d43795c5c0cd8fd7fcb5313432270577bdecbcbc617a8214954") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Control Structures Example"))) (kind "package") (name "Control Structures Example") (declared-name "Control Structures Example") (range (start (line 0) (character 0)) (end (line 0) (character 599))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Control Structures Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::AddCharge"))) (kind "action def") (name "AddCharge") (declared-name "AddCharge") (range (start (line 9) (character 1)) (end (line 9) (character 43))) (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind "in out parameter") (name "charge") (declared-name "charge") (range (start (line 9) (character 24)) (end (line 9) (character 41))) (parent (node (document "d0") (qualified-name "Control Structures Example::AddCharge"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::BatteryCharged"))) (kind "attribute def") (name "BatteryCharged") (declared-name "BatteryCharged") (range (start (line 3) (character 1)) (end (line 3) (character 30))) (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (kind "action def") (name "ChargeBattery") (declared-name "ChargeBattery") (range (start (line 12) (character 1)) (end (line 12) (character 333))) (parent (node (document "d0") (qualified-name "Control Structures Example"))) (authored (membership (kind Owning)) (relationships (perform (reference "Control Structures Example::ChargeBattery::endCharging") (range none)))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind "action") (name "endCharging") (declared-name "endCharging") (range (start (line 25) (character 2)) (end (line 25) (character 40))) (parent (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (authored (relationships (typing (reference "EndCharging") (range none)) (flow (reference "Control Structures Example::ChargeBattery::done") (range none)))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::EndCharging"))) (kind "action def") (name "EndCharging") (declared-name "EndCharging") (range (start (line 10) (character 1)) (end (line 10) (character 24))) (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery"))) (kind "action def") (name "MonitorBattery") (declared-name "MonitorBattery") (range (start (line 8) (character 1)) (end (line 8) (character 49))) (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind "in out parameter") (name "charge") (declared-name "charge") (range (start (line 8) (character 29)) (end (line 8) (character 47))) (parent (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::battery"))) (kind "part") (name "battery") (declared-name "battery") (range (start (line 5) (character 1)) (end (line 5) (character 14))) (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::powerSystem"))) (kind "part") (name "powerSystem") (declared-name "powerSystem") (range (start (line 6) (character 1)) (end (line 6) (character 18))) (parent (node (document "d0") (qualified-name "Control Structures Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (kind performSource) (ordinal 0)) (authored-target "Control Structures Example::ChargeBattery::endCharging") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging")))))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)) (authored-target "EndCharging") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Control Structures Example::EndCharging")))))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind flowSource) (ordinal 0)) (authored-target "Control Structures Example::ChargeBattery::done") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (target (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (target (node (document "d0") (qualified-name "Control Structures Example::EndCharging"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
