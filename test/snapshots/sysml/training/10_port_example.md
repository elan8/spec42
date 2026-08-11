# META
~~~ini
description=SysML Training 10 (Ports): Port Example
type=file
~~~
# SOURCE
~~~sysml
package 'Port Example' {
	
	attribute def Temp;
	
	part def Fuel;
	
	port def FuelOutPort {
		attribute temperature : Temp;
		out item fuelSupply : Fuel;
		in item fuelReturn : Fuel;
	}
	
	port def FuelInPort {
		attribute temperature : Temp;
		in item fuelSupply : Fuel;
		out item fuelReturn : Fuel;
	}
	
	part def FuelTankAssembly {
		port fuelTankPort : FuelOutPort;
	}
	
	part def Engine {
		port engineFuelPort : FuelInPort;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10_port_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Port Example' {

    attribute def Temp;

    part def Fuel;

    port def FuelOutPort {
        attribute temperature : Temp;
        out item fuelSupply : Fuel;
        in item fuelReturn : Fuel;
    }

    port def FuelInPort {
        attribute temperature : Temp;
        in item fuelSupply : Fuel;
        out item fuelReturn : Fuel;
    }

    part def FuelTankAssembly {
        port fuelTankPort : FuelOutPort;
    }

    part def Engine {
        port engineFuelPort : FuelInPort;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f556f5bd429a9ca94ec5cdd7a4357cf54980c098ae87ce2a150fa78162ad1768") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Port Example"))) (kind "package") (name "Port Example") (declared-name "Port Example") (range (start (line 0) (character 0)) (end (line 0) (character 435))))
    (element (id (node (document "d0") (qualified-name "Port Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 22) (character 1)) (end (line 22) (character 57))) (parent (node (document "d0") (qualified-name "Port Example"))))
    (element (id (node (document "d0") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind "port") (name "engineFuelPort") (declared-name "engineFuelPort") (range (start (line 23) (character 2)) (end (line 23) (character 35))) (parent (node (document "d0") (qualified-name "Port Example::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelInPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Example::Fuel"))) (kind "part def") (name "Fuel") (declared-name "Fuel") (range (start (line 4) (character 1)) (end (line 4) (character 15))) (parent (node (document "d0") (qualified-name "Port Example"))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelInPort"))) (kind "port def") (name "FuelInPort") (declared-name "FuelInPort") (range (start (line 12) (character 1)) (end (line 12) (character 116))) (parent (node (document "d0") (qualified-name "Port Example"))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (kind "item") (name "fuelReturn") (declared-name "fuelReturn") (range (start (line 15) (character 2)) (end (line 15) (character 29))) (parent (node (document "d0") (qualified-name "Port Example::FuelInPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (kind "item") (name "fuelSupply") (declared-name "fuelSupply") (range (start (line 14) (character 2)) (end (line 14) (character 28))) (parent (node (document "d0") (qualified-name "Port Example::FuelInPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (kind "attribute") (name "temperature") (declared-name "temperature") (range (start (line 13) (character 2)) (end (line 13) (character 31))) (parent (node (document "d0") (qualified-name "Port Example::FuelInPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Temp") (range none)) (typing (reference "Temp") (range (start (line 13) (character 26)) (end (line 13) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelInPort::~FuelInPort"))) (kind "conjugated port definition") (name "~FuelInPort") (declared-name "~FuelInPort") (range (start (line 12) (character 1)) (end (line 12) (character 116))) (parent (node (document "d0") (qualified-name "Port Example::FuelInPort"))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelOutPort"))) (kind "port def") (name "FuelOutPort") (declared-name "FuelOutPort") (range (start (line 6) (character 1)) (end (line 6) (character 117))) (parent (node (document "d0") (qualified-name "Port Example"))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (kind "item") (name "fuelReturn") (declared-name "fuelReturn") (range (start (line 9) (character 2)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "Port Example::FuelOutPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (kind "item") (name "fuelSupply") (declared-name "fuelSupply") (range (start (line 8) (character 2)) (end (line 8) (character 29))) (parent (node (document "d0") (qualified-name "Port Example::FuelOutPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind "attribute") (name "temperature") (declared-name "temperature") (range (start (line 7) (character 2)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "Port Example::FuelOutPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Temp") (range none)) (typing (reference "Temp") (range (start (line 7) (character 26)) (end (line 7) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::~FuelOutPort"))) (kind "conjugated port definition") (name "~FuelOutPort") (declared-name "~FuelOutPort") (range (start (line 6) (character 1)) (end (line 6) (character 117))) (parent (node (document "d0") (qualified-name "Port Example::FuelOutPort"))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelTankAssembly"))) (kind "part def") (name "FuelTankAssembly") (declared-name "FuelTankAssembly") (range (start (line 18) (character 1)) (end (line 18) (character 66))) (parent (node (document "d0") (qualified-name "Port Example"))))
    (element (id (node (document "d0") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind "port") (name "fuelTankPort") (declared-name "fuelTankPort") (range (start (line 19) (character 2)) (end (line 19) (character 34))) (parent (node (document "d0") (qualified-name "Port Example::FuelTankAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelOutPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Example::Temp"))) (kind "attribute def") (name "Temp") (declared-name "Temp") (range (start (line 2) (character 1)) (end (line 2) (character 20))) (parent (node (document "d0") (qualified-name "Port Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelInPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::FuelInPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Temp") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 1)) (authored-target "Temp") (range (start (line 13) (character 26)) (end (line 13) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Temp") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 1)) (authored-target "Temp") (range (start (line 7) (character 26)) (end (line 7) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelOutPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Example::FuelOutPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::Engine::engineFuelPort"))) (target (node (document "d0") (qualified-name "Port Example::FuelInPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (target (node (document "d0") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (target (node (document "d0") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (target (node (document "d0") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (target (node (document "d0") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (target (node (document "d0") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (target (node (document "d0") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (target (node (document "d0") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (target (node (document "d0") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (target (node (document "d0") (qualified-name "Port Example::FuelOutPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
