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
        (code "port_owned_usage_composite")
        (source "semantic")
        (range (start 8 2) (end 8 29))
      )
      (diagnostic
        (severity warning)
        (code "port_owned_usage_composite")
        (source "semantic")
        (range (start 9 2) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "port_owned_usage_composite")
        (source "semantic")
        (range (start 14 2) (end 14 28))
      )
      (diagnostic
        (severity warning)
        (code "port_owned_usage_composite")
        (source "semantic")
        (range (start 15 2) (end 15 29))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 19 2) (end 19 34))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 23 2) (end 23 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4e5e022421faaf34783c440a612d48a28954703b6e5c82f9d328a0e83223089a") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelInPort")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Temp")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Temp")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelOutPort")))))
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelInPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelOutPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn")) (scopes any))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply")) (scopes any))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn")) (scopes any))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Engine::engineFuelPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort")))
      (featured-by (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly")))
      (type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature")) (scopes any))
      (subtype (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature")) (scopes any))
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
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 15 24) (end 15 28)) (probe (position 15 24))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    )
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 14 23) (end 14 27)) (probe (position 14 23))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    )
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 13 26) (end 13 30)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelInPort::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
    )
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 9 23) (end 9 27)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    )
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 8 24) (end 8 28)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Fuel")))))
    )
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 7 26) (end 7 30)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::Temp")))))
    )
  )
  (query (document "memory://snapshot/10_port_example.md") (range (start 19 22) (end 19 33)) (probe (position 19 22))
    (reference (id (source (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelOutPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_example.md") (qualified-name "Port Example::FuelOutPort")))))
    )
  )
)
~~~
