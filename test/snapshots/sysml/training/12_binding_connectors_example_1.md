# META
~~~ini
description=SysML Training 12 (Binding Connectors): Binding Connectors Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Binding Connectors Example-1' {
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
			
			bind fuelTankPort.fuelSupply = pump.pumpOut;
			bind fuelTankPort.fuelReturn = tank.fuelIn;
			
			part pump : FuelPump {
				out item pumpOut : Fuel;
				in item pumpIn : Fuel;
			}
			
			part tank : FuelTank {
				out item fuelOut : Fuel;
				in item fuelIn : Fuel;
			}
		}
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12_binding_connectors_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 14) (end 8 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 8) (end 14 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 34) (end 14 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 8) (end 15 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 34) (end 15 45))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 18 4) (end 18 59))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 23 4) (end 23 59))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "487d9275e70437fe918487973fafec741e5d9e6a98c686c4ccdd83d249bdf8ee") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1"))) (kind "package") (name "Binding Connectors Example-1") (declared-name "Binding Connectors Example-1"))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::FuelPump"))) (kind "part def") (name "FuelPump") (declared-name "FuelPump") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1"))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1"))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1"))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (kind "part") (name "tank") (declared-name "tank") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly")))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort"))) (kind "port") (name "fuelTankPort") (declared-name "fuelTankPort") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelTankPort")))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::pump"))) (kind "part") (name "pump") (declared-name "pump") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPump")))))
    (element (id (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::tank"))) (kind "part") (name "tank") (declared-name "tank") (parent (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTank")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (kind bindSource) (ordinal 0)) (authored-target "fuelTankPort::fuelSupply") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (kind bindSource) (ordinal 1)) (authored-target "fuelTankPort::fuelReturn") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (kind bindTarget) (ordinal 0)) (authored-target "pump::pumpOut") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))) (kind bindTarget) (ordinal 1)) (authored-target "tank::fuelIn") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelTankPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::pump"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPump") (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::FuelPump")))))
    (reference (id (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::tank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTank") (outcome (status resolved) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::FuelTank")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::pump"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::FuelPump"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::pump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::tank"))) (target (node (document "d0") (qualified-name "Binding Connectors Example-1::FuelTank"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::tank"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 16) (end 7 23)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 7 16) (end 7 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Binding Connectors Example-1::Vehicle") (range (start 3 1) (end 3 18)))
        )
      )
    )
    (query (range (start 17 15) (end 17 23)) (probe (position 17 15))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::pump"))
        (kind featureTyping) (ordinal 0) (authored-target "FuelPump")
        (range (start 17 15) (end 17 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Binding Connectors Example-1::FuelPump") (range (start 4 1) (end 4 19)))
        )
      )
    )
    (query (range (start 22 15) (end 22 23)) (probe (position 22 15))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::tank"))
        (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
        (range (start 22 15) (end 22 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Binding Connectors Example-1::FuelTank") (range (start 5 1) (end 5 19)))
        )
      )
    )
    (query (range (start 15 34) (end 15 45)) (probe (position 15 34))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))
        (kind bindTarget) (ordinal 1) (authored-target "tank::fuelIn")
        (range (start 15 34) (end 15 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 18) (end 9 30)) (probe (position 9 18))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort"))
        (kind redefinition) (ordinal 0) (authored-target "fuelTankPort")
        (range (start 9 18) (end 9 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank::fuelTankPort") (range (start 9 3) (end 9 106)))
        )
      )
    )
    (query (range (start 14 34) (end 14 46)) (probe (position 14 34))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))
        (kind bindTarget) (ordinal 0) (authored-target "pump::pumpOut")
        (range (start 14 34) (end 14 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 30)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Port Example::*")
        (range (start 1 16) (end 1 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 14) (end 8 30)) (probe (position 8 14))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))
        (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
        (range (start 8 14) (end 8 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 8) (end 14 31)) (probe (position 14 8))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))
        (kind bindSource) (ordinal 0) (authored-target "fuelTankPort::fuelSupply")
        (range (start 14 8) (end 14 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 8) (end 15 31)) (probe (position 15 8))
      (reference
        (source (document "d0") (qualified-name "Binding Connectors Example-1::vehicle::tank"))
        (kind bindSource) (ordinal 1) (authored-target "fuelTankPort::fuelReturn")
        (range (start 15 8) (end 15 31))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
