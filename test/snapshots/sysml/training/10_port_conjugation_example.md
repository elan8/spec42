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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a63572fe4c6c12b1696563234a7d7f2fd63309f704acd2af198c12bed4ecf8ca") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Fuel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Temp"))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort"))))
    (declaration (id (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")))))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/10_port_conjugation_example.md") (range (start 7 26) (end 7 30)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0) (authored-target "Temp")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::Temp")))))
  )
  (query (document "memory://snapshot/10_port_conjugation_example.md") (range (start 13 22) (end 13 30)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/10_port_conjugation_example.md") (qualified-name "Port Conjugation Example::FuelPort")))))
  )
)
~~~
