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
  (document "13_flow_usage_example.md"
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "835095463f09b6050ae89cba2f14e18fefd87ad9bafd044ab369adfc92d90f22") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Flow Usage Example"))) (kind "package") (name "Flow Usage Example") (declared-name "Flow Usage Example"))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Flow Usage Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Flow Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Flow Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind "part") (name "tankAssy") (declared-name "tankAssy") (parent (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Flow Usage Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "tankAssy::fuelTankPort::fuelSupply") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowSource) (ordinal 1)) (authored-target "eng::engineFuelPort::fuelReturn") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowTarget) (ordinal 0)) (authored-target "eng::engineFuelPort::fuelSupply") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind flowTarget) (ordinal 1)) (authored-target "tankAssy::fuelTankPort::fuelReturn") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (target (node (document "d0") (qualified-name "Flow Usage Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flow Usage Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 13) (end 7 19)) (probe (position 7 13))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::vehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 7 13) (end 7 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 16) (end 5 23)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 5 16) (end 5 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Flow Usage Example::Vehicle") (range (start 3 1) (end 3 18)))
        )
      )
    )
    (query (range (start 1 16) (end 1 30)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Port Example::*")
        (range (start 1 16) (end 1 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 18) (end 6 34)) (probe (position 6 18))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::vehicle::tankAssy"))
        (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
        (range (start 6 18) (end 6 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 6) (end 11 35)) (probe (position 11 6))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::vehicle"))
        (kind flowTarget) (ordinal 0) (authored-target "eng::engineFuelPort::fuelSupply")
        (range (start 11 6) (end 11 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 9) (end 14 38)) (probe (position 14 9))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::vehicle"))
        (kind flowSource) (ordinal 1) (authored-target "eng::engineFuelPort::fuelReturn")
        (range (start 14 9) (end 14 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 9) (end 10 41)) (probe (position 10 9))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::vehicle"))
        (kind flowSource) (ordinal 0) (authored-target "tankAssy::fuelTankPort::fuelSupply")
        (range (start 10 9) (end 10 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 6) (end 15 38)) (probe (position 15 6))
      (reference
        (source (document "d0") (qualified-name "Flow Usage Example::vehicle"))
        (kind flowTarget) (ordinal 1) (authored-target "tankAssy::fuelTankPort::fuelReturn")
        (range (start 15 6) (end 15 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
