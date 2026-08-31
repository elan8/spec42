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
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 1) (end 5 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 1) (end 6 18))
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
        (range (start 26 7) (end 26 11))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:97d2e7b7546d1737f5c0d55aa38ee1a684a6a20a7c8dc5ee115b9c299d48de59"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::BatteryCharged"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0))))) (kind loop) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "done")))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "monitor::charge")))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AddCharge")))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (value (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "monitor::charge")))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MonitorBattery")))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EndCharging")))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction out)))))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::battery"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::powerSystem"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "monitor::charge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge")))))))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge"))))) (kind featureTyping) (ordinal 0))
      (authored-target "AddCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge")))))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "monitor::charge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge")))))))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor"))))) (kind featureTyping) (ordinal 0))
      (authored-target "MonitorBattery")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery")))))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0))
      (authored-target "EndCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging")))))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge"))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge"))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge"))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor"))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge"))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge"))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge"))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor"))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge"))))) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge")))
      (subtype (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge")))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge")))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery")))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)))))
      (type (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge")) (provenance authored))
      (effective-type (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge")) (source direct))
      (supertype (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")))))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (subtype (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge")))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)))))
      (type (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery")) (provenance authored))
      (effective-type (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery")) (source direct))
      (supertype (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge")))))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")))))
      (subtype (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging")))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery")))
      (type (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging")) (provenance authored))
      (effective-type (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging")) (source direct))
      (supertype (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging")))
      (subtype (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery")))
      (subtype (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge")))
      (featured-by (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 9 36) (end 9 40)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 26 7) (end 26 11)) (probe (position 26 7))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 18 11) (end 18 25)) (probe (position 18 11))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "monitor::charge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge")))))))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 19 23) (end 19 32)) (probe (position 19 23))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge"))))) (kind featureTyping) (ordinal 0) (authored-target "AddCharge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::AddCharge")))))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 20 17) (end 20 31)) (probe (position 20 17))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "addCharge")) (named (kind parameter) (name "charge")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "monitor::charge")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor")) (named (kind parameter) (name "charge")))))))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 14 20) (end 14 34)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (path (named (kind package) (name "Control Structures Example")) (named (kind action-def) (name "ChargeBattery")) (anonymous (kind loop) (ordinal 0)) (named (kind action) (name "monitor"))))) (kind featureTyping) (ordinal 0) (authored-target "MonitorBattery")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery")))))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 25 28) (end 25 39)) (probe (position 25 28))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::ChargeBattery::endCharging"))) (kind featureTyping) (ordinal 0) (authored-target "EndCharging")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::EndCharging")))))
    )
  )
  (query (document "memory://snapshot/17_control_structures_example.md") (range (start 8 42) (end 8 46)) (probe (position 8 42))
    (reference (id (source (node (document "memory://snapshot/17_control_structures_example.md") (qualified-name "Control Structures Example::MonitorBattery::charge"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
