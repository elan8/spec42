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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1d29daa0f52c5d43795c5c0cd8fd7fcb5313432270577bdecbcbc617a8214954") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Control Structures Example"))) (kind "package") (name "Control Structures Example") (declared-name "Control Structures Example"))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Control Structures Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::AddCharge"))) (kind "action def") (name "AddCharge") (declared-name "AddCharge") (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind "in out parameter") (name "charge") (declared-name "charge") (parent (node (document "d0") (qualified-name "Control Structures Example::AddCharge"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::BatteryCharged"))) (kind "attribute def") (name "BatteryCharged") (declared-name "BatteryCharged") (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (kind "action def") (name "ChargeBattery") (declared-name "ChargeBattery") (parent (node (document "d0") (qualified-name "Control Structures Example"))) (authored (membership (kind Owning)) (relationships (perform (reference "Control Structures Example::ChargeBattery::endCharging")))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind "action") (name "endCharging") (declared-name "endCharging") (parent (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (authored (relationships (typing (reference "EndCharging")) (flow (reference "Control Structures Example::ChargeBattery::done")))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::EndCharging"))) (kind "action def") (name "EndCharging") (declared-name "EndCharging") (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery"))) (kind "action def") (name "MonitorBattery") (declared-name "MonitorBattery") (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind "in out parameter") (name "charge") (declared-name "charge") (parent (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::battery"))) (kind "part") (name "battery") (declared-name "battery") (parent (node (document "d0") (qualified-name "Control Structures Example"))))
    (element (id (node (document "d0") (qualified-name "Control Structures Example::powerSystem"))) (kind "part") (name "powerSystem") (declared-name "powerSystem") (parent (node (document "d0") (qualified-name "Control Structures Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (kind performSource) (ordinal 0)) (authored-target "Control Structures Example::ChargeBattery::endCharging") (outcome (status resolved) (target (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging")))))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)) (authored-target "EndCharging") (outcome (status resolved) (target (node (document "d0") (qualified-name "Control Structures Example::EndCharging")))))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind flowSource) (ordinal 0)) (authored-target "Control Structures Example::ChargeBattery::done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (target (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (target (node (document "d0") (qualified-name "Control Structures Example::EndCharging"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Control Structures Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
