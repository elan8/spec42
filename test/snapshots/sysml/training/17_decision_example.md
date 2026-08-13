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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/17_decision_example.md"
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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 8) (end 13 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 17 2) (end 19 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 22 3) (end 22 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 23 3) (end 23 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 28 2) (end 28 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 31 2) (end 31 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:b4c682eb8f56fcb40c60c69fefa6cf3b1c10ee71080937769745bb2552e2c6d2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::BatteryCharged"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start"))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AddCharge"))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge::charge"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EndCharging"))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction out))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::battery"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::powerSystem"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0))
      (authored-target "AddCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")))))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0))
      (authored-target "EndCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")))))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_decision_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 9 36) (end 9 40)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 13 8) (end 13 13)) (probe (position 13 8))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 25 21) (end 25 30)) (probe (position 25 21))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0) (authored-target "AddCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")))))
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 30 23) (end 30 34)) (probe (position 30 23))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0) (authored-target "EndCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")))))
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 8 42) (end 8 46)) (probe (position 8 42))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
