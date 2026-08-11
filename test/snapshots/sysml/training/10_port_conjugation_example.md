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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Port Conjugation Example''
    (attribute_def 'Temp')
    (part_def 'Fuel')
    (port_def 'FuelPort'
      (attribute_usage 'temperature' : 'Temp')
      (item_usage out 'fuelSupply' : 'Fuel')
      (item_usage in 'fuelReturn' : 'Fuel'))
    (part_def 'FuelTank'
      (port_usage 'fuelTankPort' : 'FuelPort'))
    (part_def 'Engine'
      (port_usage 'engineFuelPort' : ~'FuelPort'))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8e9321e60830dfc6149560e45a640e2b03f69c01a9e268b5f40e6a6995335be8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example"))) (kind "package") (name "Port Conjugation Example") (declared-name "Port Conjugation Example") (range (start (line 0) (character 0)) (end (line 0) (character 313))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 16) (character 1)) (end (line 16) (character 56))) (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind "port") (name "engineFuelPort") (declared-name "engineFuelPort") (range (start (line 17) (character 2)) (end (line 17) (character 34))) (parent (node (document "d0") (qualified-name "Port Conjugation Example::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (kind "part def") (name "Fuel") (declared-name "Fuel") (range (start (line 4) (character 1)) (end (line 4) (character 15))) (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (kind "port def") (name "FuelPort") (declared-name "FuelPort") (range (start (line 6) (character 1)) (end (line 6) (character 114))) (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind "item") (name "fuelReturn") (declared-name "fuelReturn") (range (start (line 9) (character 2)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind "item") (name "fuelSupply") (declared-name "fuelSupply") (range (start (line 8) (character 2)) (end (line 8) (character 29))) (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind "attribute") (name "temperature") (declared-name "temperature") (range (start (line 7) (character 2)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Temp") (range none)) (typing (reference "Temp") (range (start (line 7) (character 26)) (end (line 7) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (kind "conjugated port definition") (name "~FuelPort") (declared-name "~FuelPort") (range (start (line 6) (character 1)) (end (line 6) (character 114))) (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (range (start (line 12) (character 1)) (end (line 12) (character 55))) (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind "port") (name "fuelTankPort") (declared-name "fuelTankPort") (range (start (line 13) (character 2)) (end (line 13) (character 31))) (parent (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (kind "attribute def") (name "Temp") (declared-name "Temp") (range (start (line 2) (character 1)) (end (line 2) (character 20))) (parent (node (document "d0") (qualified-name "Port Conjugation Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Temp") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (kind featureTyping) (ordinal 1)) (authored-target "Temp") (range (start (line 7) (character 26)) (end (line 7) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::Temp")))))
    (reference (id (source (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
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
