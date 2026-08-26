# META
~~~ini
description=SysML Training 13 (Flows): Flow Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Usage Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	part vehicle : Vehicle {
		part tankAssy : FuelTankAssembly;
		part eng : Engine;
		
		flow of Fuel
		  from tankAssy.fuelTankPort.fuelSupply
			to eng.engineFuelPort.fuelSupply;
			
		flow of Fuel
		  from eng.engineFuelPort.fuelReturn
			to tankAssy.fuelTankPort.fuelReturn;
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/13_flow_usage_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 18) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 13) (end 7 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 10) (end 9 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 9) (end 10 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 6) (end 11 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 10) (end 13 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 9) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 6) (end 15 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d6f7f1cdc4b131fde732467bfbabd5a78735156842662d0a4ce3bc2a082f0719") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Port Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "tankAssy::fuelTankPort::fuelSupply")) (flowTarget (reference "eng::engineFuelPort::fuelSupply")) (flowPayloadType (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "eng::engineFuelPort::fuelReturn")) (flowTarget (reference "tankAssy::fuelTankPort::fuelReturn")) (flowPayloadType (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTankAssembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Port Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "tankAssy::fuelTankPort::fuelSupply")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0))
      (authored-target "eng::engineFuelPort::fuelReturn")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "eng::engineFuelPort::fuelSupply")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0))
      (authored-target "tankAssy::fuelTankPort::fuelReturn")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (kind flowPayloadType) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::eng"))) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle")))
      (subtype (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle")))
      (type (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::eng")))
      (featured-by (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::tankAssy")))
      (featured-by (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 1 16) (end 1 33)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Port Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 5 16) (end 5 23)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 10 9) (end 10 41)) (probe (position 10 9))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "tankAssy::fuelTankPort::fuelSupply")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 14 9) (end 14 38)) (probe (position 14 9))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0) (authored-target "eng::engineFuelPort::fuelReturn")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 11 6) (end 11 35)) (probe (position 11 6))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "eng::engineFuelPort::fuelSupply")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 15 6) (end 15 38)) (probe (position 15 6))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0) (authored-target "tankAssy::fuelTankPort::fuelReturn")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 9 10) (end 9 14)) (probe (position 9 10))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0) (authored-target "Fuel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 13 10) (end 13 14)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (named (kind part) (name "vehicle")) (anonymous (kind flow) (ordinal 1))))) (kind flowPayloadType) (ordinal 0) (authored-target "Fuel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 7 13) (end 7 19)) (probe (position 7 13))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_usage_example.md") (range (start 6 18) (end 6 34)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
    )
  )
)
~~~
