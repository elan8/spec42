# META
~~~ini
description=SysML Training 12 (Binding Connectors): Binding Connectors Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Binding Connectors Example-2' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	part def FuelPump;
	part def FuelTank;
	
	part vehicle : Vehicle {	
		part tank : FuelTankAssembly {
			port redefines fuelTankPort {
				out item redefines fuelSupply;
				in item redefines fuelReturn;
			}
			
			part pump : FuelPump {
				out item pumpOut : Fuel = fuelTankPort.fuelSupply;
				in item pumpIn : Fuel;
			}
			
			part tank : FuelTank {
				out item fuelOut : Fuel;
				in item fuelIn : Fuel = fuelTankPort.fuelReturn;
			}
		}
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/12_binding_connectors_example_2.md"
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
        (range (start 8 14) (end 8 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 9 18) (end 9 30))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 10 4) (end 11 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 10 4) (end 11 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 15 4) (end 17 3))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 20 4) (end 22 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b51e37e605cf5d9a3045cd4da28c46a1b46ceffb31ede08b9954e86107d73880") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Port Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelPump"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelTank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTankAssembly"))))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelTankPort"))))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPump"))))
    (declaration (id (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTank"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Port Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelTankPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPump")
      (outcome (status resolved) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelPump")))))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelTank")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle"))) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelPump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelTank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/12_binding_connectors_example_2.md") (range (start 1 16) (end 1 33)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Port Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/12_binding_connectors_example_2.md") (range (start 7 16) (end 7 23)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::Vehicle")))))
  )
  (query (document "memory://snapshot/12_binding_connectors_example_2.md") (range (start 8 14) (end 8 30)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/12_binding_connectors_example_2.md") (range (start 9 18) (end 9 30)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelTankPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/12_binding_connectors_example_2.md") (range (start 14 15) (end 14 23)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPump")
      (outcome (status resolved) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelPump")))))
  )
  (query (document "memory://snapshot/12_binding_connectors_example_2.md") (range (start 19 15) (end 19 23)) (probe (position 19 15))
    (reference (id (source (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/12_binding_connectors_example_2.md") (qualified-name "Binding Connectors Example-2::FuelTank")))))
  )
)
~~~
