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
            (element (kind "port") (id (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (name "engineFuelPort") (declared-name "engineFuelPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))) (name "Fuel") (declared-name "Fuel") (declared))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))) (name "FuelPort") (declared-name "FuelPort")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (name "fuelReturn") (declared-name "fuelReturn") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (name "fuelSupply") (declared-name "fuelSupply") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (name "temperature") (declared-name "temperature") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (name "~FuelPort") (declared-name "~FuelPort") (effective (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank"))) (name "FuelTank") (declared-name "FuelTank") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (name "fuelTankPort") (declared-name "fuelTankPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))) (name "Temp") (declared-name "Temp") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::Engine::engineFuelPort"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::~FuelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelReturn"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::fuelSupply"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::Fuel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort::temperature"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::Temp"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Conjugation Example::FuelTank::fuelTankPort"))) (to (node (document "d0") (qualified-name "Port Conjugation Example::FuelPort"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
