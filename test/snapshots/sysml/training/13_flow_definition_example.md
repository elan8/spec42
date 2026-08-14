# META
~~~ini
description=SysML Training 13 (Flows): Flow Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Definition Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	flow def FuelFlow {
		ref :>> payload : Fuel;
		end port supplierPort : FuelOutPort;
		end port consumerPort : FuelInPort;
	}
	
	part vehicle : Vehicle {
		part tankAssy : FuelTankAssembly;
		part eng : Engine;
		
		flow : FuelFlow of Fuel
		  from tankAssy.fuelTankPort.fuelSupply
			to eng.engineFuelPort.fuelSupply;
			
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/13_flow_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 6 2) (end 6 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 26) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 26) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 18) (end 12 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 13) (end 13 19))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 15 2) (end 19 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:edf480a47678c7d31ed14319668b48abff2d01f1d173188265f73e3684d4aefd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Port Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::FuelFlow"))) (kind flow-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::FuelFlow::consumerPort"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelInPort"))))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::FuelFlow::supplierPort"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelOutPort"))))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle::tankAssy"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTankAssembly"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Port Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::FuelFlow::consumerPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelInPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::FuelFlow::supplierPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelOutPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle"))) (target (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/13_flow_definition_example.md") (range (start 1 16) (end 1 33)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Port Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13_flow_definition_example.md") (range (start 8 26) (end 8 36)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::FuelFlow::consumerPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelInPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13_flow_definition_example.md") (range (start 7 26) (end 7 37)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::FuelFlow::supplierPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelOutPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13_flow_definition_example.md") (range (start 11 16) (end 11 23)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::Vehicle")))))
  )
  (query (document "memory://snapshot/13_flow_definition_example.md") (range (start 13 13) (end 13 19)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13_flow_definition_example.md") (range (start 12 18) (end 12 34)) (probe (position 12 18))
    (reference (id (source (node (document "memory://snapshot/13_flow_definition_example.md") (qualified-name "Flow Definition Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
  )
)
~~~
