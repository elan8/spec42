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
  (document "memory://snapshot/17_control_structures_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 42) (end 8 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 36) (end 9 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 13 2) (end 23 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 23 4) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 25 2) (end 25 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 26 2) (end 26 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:97d2e7b7546d1737f5c0d55aa38ee1a684a6a20a7c8dc5ee115b9c299d48de59") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::BatteryCharged"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction out))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::battery"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::powerSystem"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 9 36) (end 9 40)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 8 42) (end 8 46)) (probe (position 8 42))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
