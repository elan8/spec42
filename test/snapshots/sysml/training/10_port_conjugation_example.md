# META
~~~ini
description=SysML Training 10 (Ports): Port Conjugation Example
type=file
~~~
# SOURCE
~~~sysml
package 'Port Conjugation Example' {
	
	attribute def Temp;
	
	part def Fuel;
	
	port def FuelPort {
		attribute temperature : Temp;
		out item fuelSupply : Fuel;
		in item fuelReturn : Fuel;
	}
	
	part def FuelTank {
		port fuelTankPort : FuelPort;
	}
	
	part def Engine {
		port engineFuelPort : ~FuelPort;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10_port_conjugation_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8e9321e60830dfc6149560e45a640e2b03f69c01a9e268b5f40e6a6995335be8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example"))) (kind "package") (name "Port Conjugation Example") (declared-name "Port Conjugation Example"))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind "port") (name "engineFuelPort") (declared-name "engineFuelPort") (parent (node (document "d0") (qualified-name "Port Conjugation Example::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort")))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (kind "part def") (name "Fuel") (declared-name "Fuel") (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (kind "port def") (name "FuelPort") (declared-name "FuelPort") (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind "item") (name "fuelReturn") (declared-name "fuelReturn") (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel")))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind "item") (name "fuelSupply") (declared-name "fuelSupply") (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel")))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind "attribute") (name "temperature") (declared-name "temperature") (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Temp")) (typing (reference "Temp")))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (kind "conjugated port definition") (name "~FuelPort") (declared-name "~FuelPort") (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind "port") (name "fuelTankPort") (declared-name "fuelTankPort") (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort")))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (kind "attribute def") (name "Temp") (declared-name "Temp") (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Temp") (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 1)) (authored-target "Temp") (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (target (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (target (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (target (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (target (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (target (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (target (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 26) (end 7 30)) (probe (position 7 26))
      (reference
        (source (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))
        (kind featureTyping) (ordinal 1) (authored-target "Temp")
        (range (start 7 26) (end 7 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Port Conjugation Example::Temp") (range (start 2 1) (end 2 20)))
        )
      )
    )
  )
)
~~~
