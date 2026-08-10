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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Port Conjugation Example"))) (name "Port Conjugation Example") (declared-name "Port Conjugation Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Conjugation Example::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (name "engineFuelPort") (declared-name "engineFuelPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (name "Fuel") (declared-name "Fuel") (declared))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (name "FuelPort") (declared-name "FuelPort")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (name "fuelReturn") (declared-name "fuelReturn") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (name "fuelSupply") (declared-name "fuelSupply") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (name "temperature") (declared-name "temperature") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (name "~FuelPort") (declared-name "~FuelPort") (effective (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank"))) (name "FuelTank") (declared-name "FuelTank") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (name "fuelTankPort") (declared-name "fuelTankPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (name "Temp") (declared-name "Temp") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/10_port_conjugation_example.md"
    (diagnostics
    )
  )
)
~~~
