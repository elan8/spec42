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
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 9 2) (end 11 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 13 2) (end 15 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d6f7f1cdc4b131fde732467bfbabd5a78735156842662d0a4ce3bc2a082f0719") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (path (named (kind package) (name "Flow Usage Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Port Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
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
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (target (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::vehicle")))
      (supertype (node (document "memory://snapshot/13_flow_usage_example.md") (qualified-name "Flow Usage Example::Vehicle")) (scopes any))
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
