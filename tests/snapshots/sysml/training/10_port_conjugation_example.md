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
  (document "memory://snapshot/10_port_conjugation_example.md"
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
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 13 2) (end 13 31))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 17 2) (end 17 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a63572fe4c6c12b1696563234a7d7f2fd63309f704acd2af198c12bed4ecf8ca") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Temp")))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort")))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")))))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort")))
      (featured-by (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine")))
      (type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")))
      (subtype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn")) (scopes any))
      (subtype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))
      (subtype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort")) (scopes any))
      (subtype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn")))
      (featured-by (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))
      (type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply")))
      (featured-by (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))
      (type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature")))
      (featured-by (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))
      (type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort")))
      (featured-by (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank")))
      (type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")) (source direct))
      (supertype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")))
      (subtype (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/10_port_conjugation_example.md") (range (start 17 25) (end 17 33)) (probe (position 17 25))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))))
    )
  )
  (query (document "memory://snapshot/10_port_conjugation_example.md") (range (start 9 23) (end 9 27)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")))))
    )
  )
  (query (document "memory://snapshot/10_port_conjugation_example.md") (range (start 8 24) (end 8 28)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel")))))
    )
  )
  (query (document "memory://snapshot/10_port_conjugation_example.md") (range (start 7 26) (end 7 30)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")))))
    )
  )
  (query (document "memory://snapshot/10_port_conjugation_example.md") (range (start 13 22) (end 13 30)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))))
    )
  )
)
~~~
