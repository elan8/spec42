# META
~~~ini
description=SysML Training 13 (Flows): Flow Interface Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
		
		flow supplierPort.fuelSupply to consumerPort.fuelSupply;			
		flow consumerPort.fuelReturn to supplierPort.fuelReturn;
	}
	
	part vehicle : Vehicle {	
		part tankAssy : FuelTankAssembly;		
		part eng : Engine;
		
		interface : FuelInterface connect 
			supplierPort ::> tankAssy.fuelTankPort to 
			consumerPort ::> eng.engineFuelPort;
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/13_flow_interface_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
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
        (range (start 6 21) (end 6 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 21) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 7) (end 9 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 34) (end 9 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 7) (end 10 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 34) (end 10 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 18) (end 14 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 13) (end 15 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 20) (end 18 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 20) (end 19 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:eb3f7e41b66825459489cd253eb4f98629a338d572f2a462a96c34403ad1baad"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Port Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "supplierPort::fuelSupply")) (flowTarget (reference "consumerPort::fuelSupply")))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 1))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "consumerPort::fuelReturn")) (flowTarget (reference "supplierPort::fuelReturn")))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::consumerPort"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelInPort")))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::supplierPort"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelOutPort")))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelInterface")) (connectorEnd (reference "tankAssy::fuelTankPort")) (connectorEnd (reference "eng::engineFuelPort")))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::tankAssy"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTankAssembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Port Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "supplierPort::fuelSupply")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0))
      (authored-target "consumerPort::fuelReturn")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "consumerPort::fuelSupply")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0))
      (authored-target "supplierPort::fuelReturn")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::consumerPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelInPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::supplierPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelOutPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")))))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "tankAssy::fuelTankPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "eng::engineFuelPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 1))))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::consumerPort"))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::supplierPort"))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::eng"))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::tankAssy"))) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::consumerPort")))
      (featured-by (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::supplierPort")))
      (featured-by (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle")))
      (subtype (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle")))
      (type (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle")))
      (type (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")) (provenance authored))
      (effective-type (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")) (source direct))
      (supertype (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::eng")))
      (featured-by (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::tankAssy")))
      (featured-by (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 1 16) (end 1 33)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Port Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 9 7) (end 9 30)) (probe (position 9 7))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "supplierPort::fuelSupply")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 10 7) (end 10 30)) (probe (position 10 7))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 1))))) (kind flowSource) (ordinal 0) (authored-target "consumerPort::fuelReturn")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 9 34) (end 9 57)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "consumerPort::fuelSupply")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 10 34) (end 10 57)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind interface-def) (name "FuelInterface")) (anonymous (kind flow) (ordinal 1))))) (kind flowTarget) (ordinal 0) (authored-target "supplierPort::fuelReturn")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 7 21) (end 7 31)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::consumerPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelInPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 6 21) (end 6 32)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface::supplierPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelOutPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 13 16) (end 13 23)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 17 14) (end 17 27)) (probe (position 17 14))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "FuelInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::FuelInterface")))))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 18 20) (end 18 41)) (probe (position 18 20))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "tankAssy::fuelTankPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 19 20) (end 19 38)) (probe (position 19 20))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (path (named (kind package) (name "Flow Interface Example")) (named (kind part) (name "vehicle")) (anonymous (kind interface) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "eng::engineFuelPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 15 13) (end 15 19)) (probe (position 15 13))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/13_flow_interface_example.md") (range (start 14 18) (end 14 34)) (probe (position 14 18))
    (reference (id (source (node (document "memory://snapshot/13_flow_interface_example.md") (qualified-name "Flow Interface Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
    )
  )
)
~~~
