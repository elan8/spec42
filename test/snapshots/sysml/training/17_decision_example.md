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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 13) (end 15 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 23) (end 18 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 7) (end 21 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 6) (end 22 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 6) (end 23 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 15) (end 26 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 7) (end 28 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 7) (end 31 11))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b4c682eb8f56fcb40c60c69fefa6cf3b1c10ee71080937769745bb2552e2c6d2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::BatteryCharged"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind merge) (ordinal 0))))) (kind merge) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (mergeInput (reference "continueCharging")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "decide")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "monitor::batteryCharge")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "monitor::batteryCharge")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "continueCharging")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 2))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "done")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "addCharge")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "endCharging")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AddCharge")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "monitor::batteryCharge")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EndCharging")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MonitorBattery")))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor::batteryCharge"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction out)))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction out)))))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::battery"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::powerSystem"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "monitor::batteryCharge")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "monitor::batteryCharge")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0))
      (authored-target "continueCharging")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "decide")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "continueCharging")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "addCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge")))))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "endCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging")))))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0))
      (authored-target "AddCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")))))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge::charge"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "monitor::batteryCharge")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0))
      (authored-target "EndCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")))))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind featureTyping) (ordinal 0))
      (authored-target "MonitorBattery")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery")))))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor::batteryCharge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor"))) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")))
      (subtype (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge::charge")))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind merge) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge")))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
      (type (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")) (provenance authored))
      (effective-type (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")) (source direct))
      (supertype (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge::charge")))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging")))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
      (type (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")) (provenance authored))
      (effective-type (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")) (source direct))
      (supertype (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor")))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery")))
      (type (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery")) (provenance authored))
      (effective-type (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery")) (source direct))
      (supertype (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor::batteryCharge")))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor")))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")))
      (subtype (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery")))
      (subtype (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery::charge")))
      (featured-by (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_decision_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 9 36) (end 9 40)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 13 8) (end 13 13)) (probe (position 13 8))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 22 6) (end 22 27)) (probe (position 22 6))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "monitor::batteryCharge")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 23 6) (end 23 27)) (probe (position 23 6))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "monitor::batteryCharge")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 15 13) (end 15 29)) (probe (position 15 13))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0) (authored-target "continueCharging")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 21 7) (end 21 13)) (probe (position 21 7))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "decide")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 28 7) (end 28 23)) (probe (position 28 7))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "continueCharging")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 31 7) (end 31 11)) (probe (position 31 7))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 22 39) (end 22 48)) (probe (position 22 39))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "addCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge")))))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 23 40) (end 23 51)) (probe (position 23 40))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (path (named (kind package) (name "Decision Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind if) (ordinal 1)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "endCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging")))))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 25 21) (end 25 30)) (probe (position 25 21))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge"))) (kind featureTyping) (ordinal 0) (authored-target "AddCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::AddCharge")))))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 26 15) (end 26 36)) (probe (position 26 15))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::addCharge::charge"))) (kind memberAccessOperand) (ordinal 0) (authored-target "monitor::batteryCharge")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 30 23) (end 30 34)) (probe (position 30 23))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0) (authored-target "EndCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::EndCharging")))))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 17 24) (end 17 38)) (probe (position 17 24))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor"))) (kind featureTyping) (ordinal 0) (authored-target "MonitorBattery")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery")))))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 18 23) (end 18 27)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::ChargeBattery::monitor::batteryCharge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_decision_example.md") (range (start 8 42) (end 8 46)) (probe (position 8 42))
    (reference (id (source (node (document "memory://snapshot/17_decision_example.md") (qualified-name "Decision Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
