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
  (document "17_decision_example.md"
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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 2) (end 13 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 2) (end 15 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 2) (end 17 73))
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
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 3) (end 26 37))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "7457f55d7a17f29f9b868ffc1bf9de9986d3943f35fcef39c96ecc03b22fb12c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Decision Example"))) (kind "package") (name "Decision Example") (declared-name "Decision Example") (range (start (line 0) (character 0)) (end (line 0) (character 687))))
    (element (id (node (document "d0") (qualified-name "Decision Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Decision Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Decision Example::AddCharge"))) (kind "action def") (name "AddCharge") (declared-name "AddCharge") (range (start (line 9) (character 1)) (end (line 9) (character 43))) (parent (node (document "d0") (qualified-name "Decision Example"))))
    (element (id (node (document "d0") (qualified-name "Decision Example::AddCharge::charge"))) (kind "in out parameter") (name "charge") (declared-name "charge") (range (start (line 9) (character 24)) (end (line 9) (character 41))) (parent (node (document "d0") (qualified-name "Decision Example::AddCharge"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::BatteryCharged"))) (kind "attribute def") (name "BatteryCharged") (declared-name "BatteryCharged") (range (start (line 3) (character 1)) (end (line 3) (character 30))) (parent (node (document "d0") (qualified-name "Decision Example"))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (kind "action def") (name "ChargeBattery") (declared-name "ChargeBattery") (range (start (line 12) (character 1)) (end (line 12) (character 431))) (parent (node (document "d0") (qualified-name "Decision Example"))) (authored (membership (kind Owning)) (relationships (perform (reference "Decision Example::ChargeBattery::monitor") (range none)) (perform (reference "Decision Example::ChargeBattery::addCharge") (range none)) (perform (reference "Decision Example::ChargeBattery::endCharging") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::_initial"))) (kind "initial") (name "_initial") (range (start (line 13) (character 2)) (end (line 13) (character 14))) (parent (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (authored (relationships (flow (reference "Decision Example::ChargeBattery::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind "action") (name "addCharge") (declared-name "addCharge") (range (start (line 25) (character 2)) (end (line 25) (character 74))) (parent (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (authored (membership (kind Feature)) (relationships (typing (reference "AddCharge") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge::charge"))) (kind "in out parameter") (name "charge") (declared-name "charge") (range (start (line 26) (character 3)) (end (line 26) (character 37))) (parent (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::continueCharging"))) (kind "merge") (name "merge") (declared-name "merge") (range (start (line 15) (character 2)) (end (line 15) (character 30))) (parent (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (authored (relationships (flow (reference "Decision Example::ChargeBattery::monitor") (range none)) (flow (reference "Decision Example::ChargeBattery::done") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind "action") (name "endCharging") (declared-name "endCharging") (range (start (line 30) (character 2)) (end (line 30) (character 35))) (parent (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (authored (membership (kind Feature)) (relationships (typing (reference "EndCharging") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind "action") (name "monitor") (declared-name "monitor") (range (start (line 17) (character 2)) (end (line 17) (character 73))) (parent (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (authored (relationships (typing (reference "MonitorBattery") (range none)) (flow (reference "Decision Example::ChargeBattery::decide") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor::batteryCharge"))) (kind "in out parameter") (name "batteryCharge") (declared-name "batteryCharge") (range (start (line 18) (character 3)) (end (line 18) (character 28))) (parent (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::EndCharging"))) (kind "action def") (name "EndCharging") (declared-name "EndCharging") (range (start (line 10) (character 1)) (end (line 10) (character 24))) (parent (node (document "d0") (qualified-name "Decision Example"))))
    (element (id (node (document "d0") (qualified-name "Decision Example::MonitorBattery"))) (kind "action def") (name "MonitorBattery") (declared-name "MonitorBattery") (range (start (line 8) (character 1)) (end (line 8) (character 49))) (parent (node (document "d0") (qualified-name "Decision Example"))))
    (element (id (node (document "d0") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind "in out parameter") (name "charge") (declared-name "charge") (range (start (line 8) (character 29)) (end (line 8) (character 47))) (parent (node (document "d0") (qualified-name "Decision Example::MonitorBattery"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Decision Example::battery"))) (kind "part") (name "battery") (declared-name "battery") (range (start (line 5) (character 1)) (end (line 5) (character 14))) (parent (node (document "d0") (qualified-name "Decision Example"))))
    (element (id (node (document "d0") (qualified-name "Decision Example::powerSystem"))) (kind "part") (name "powerSystem") (declared-name "powerSystem") (range (start (line 6) (character 1)) (end (line 6) (character 18))) (parent (node (document "d0") (qualified-name "Decision Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (kind performSource) (ordinal 0)) (authored-target "Decision Example::ChargeBattery::monitor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor")))))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (kind performSource) (ordinal 1)) (authored-target "Decision Example::ChargeBattery::addCharge") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge")))))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (kind performSource) (ordinal 2)) (authored-target "Decision Example::ChargeBattery::endCharging") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging")))))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "Decision Example::ChargeBattery::start") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0)) (authored-target "AddCharge") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Decision Example::AddCharge")))))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge::charge"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::continueCharging"))) (kind flowSource) (ordinal 0)) (authored-target "Decision Example::ChargeBattery::monitor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor")))))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::continueCharging"))) (kind flowSource) (ordinal 1)) (authored-target "Decision Example::ChargeBattery::done") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)) (authored-target "EndCharging") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Decision Example::EndCharging")))))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind featureTyping) (ordinal 0)) (authored-target "MonitorBattery") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Decision Example::MonitorBattery")))))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind flowSource) (ordinal 0)) (authored-target "Decision Example::ChargeBattery::decide") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor::batteryCharge"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (target (node (document "d0") (qualified-name "Decision Example::AddCharge"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::continueCharging"))) (target (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::continueCharging"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (target (node (document "d0") (qualified-name "Decision Example::EndCharging"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (target (node (document "d0") (qualified-name "Decision Example::MonitorBattery"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Decision Example::ChargeBattery::addCharge::charge")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Decision Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
