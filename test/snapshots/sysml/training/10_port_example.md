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
  (document "memory://snapshot/10_port_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 8 2) (end 8 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 9 2) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 14 2) (end 14 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:4e5e022421faaf34783c440a612d48a28954703b6e5c82f9d328a0e83223089a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelInPort"))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Temp"))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Temp"))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelOutPort"))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelInPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelOutPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/10_port_example.md") (range (start 23 24) (end 23 34)) (probe (position 23 24))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelInPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")))))
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 13 26) (end 13 30)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 7 26) (end 7 30)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 19 22) (end 19 33)) (probe (position 19 22))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelOutPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))))
  )
)
~~~
