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
KwPort,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Port Example''
    (attribute_def 'Temp')
    (part_def 'Fuel')
    (port_def 'FuelOutPort'
      (attribute_usage 'temperature' : 'Temp')
      (item_usage out 'fuelSupply' : 'Fuel')
      (item_usage in 'fuelReturn' : 'Fuel'))
    (port_def 'FuelInPort'
      (attribute_usage 'temperature' : 'Temp')
      (item_usage in 'fuelSupply' : 'Fuel')
      (item_usage out 'fuelReturn' : 'Fuel'))
    (part_def 'FuelTankAssembly'
      (port_usage 'fuelTankPort' : 'FuelOutPort'))
    (part_def 'Engine'
      (port_usage 'engineFuelPort' : 'FuelInPort'))))
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Port Example"))) (name "Port Example") (declared-name "Port Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Example::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Port Example::Engine::engineFuelPort"))) (name "engineFuelPort") (declared-name "engineFuelPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Port Example::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Example::Fuel"))) (name "Fuel") (declared-name "Fuel") (declared))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Port Example::FuelInPort"))) (name "FuelInPort") (declared-name "FuelInPort")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (name "fuelReturn") (declared-name "fuelReturn") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Port Example::FuelInPort")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (name "fuelSupply") (declared-name "fuelSupply") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Port Example::FuelInPort")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (name "temperature") (declared-name "temperature") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Port Example::FuelInPort")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Port Example::FuelInPort::~FuelInPort"))) (name "~FuelInPort") (declared-name "~FuelInPort") (effective (featuring-type (node (document "d0") (qualified-name "Port Example::FuelInPort")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "Port Example::FuelOutPort"))) (name "FuelOutPort") (declared-name "FuelOutPort")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (name "fuelReturn") (declared-name "fuelReturn") (declared (properties (direction "in") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Port Example::FuelOutPort")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (name "fuelSupply") (declared-name "fuelSupply") (declared (properties (direction "out") (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Port Example::FuelOutPort")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (name "temperature") (declared-name "temperature") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Port Example::FuelOutPort")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Port Example::FuelOutPort::~FuelOutPort"))) (name "~FuelOutPort") (declared-name "~FuelOutPort") (effective (featuring-type (node (document "d0") (qualified-name "Port Example::FuelOutPort")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Port Example::FuelTankAssembly"))) (name "FuelTankAssembly") (declared-name "FuelTankAssembly") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (name "fuelTankPort") (declared-name "fuelTankPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Port Example::FuelTankAssembly")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Port Example::Temp"))) (name "Temp") (declared-name "Temp") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelInPort::~FuelInPort"))) (to (node (document "d0") (qualified-name "Port Example::FuelInPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelOutPort::~FuelOutPort"))) (to (node (document "d0") (qualified-name "Port Example::FuelOutPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::Engine::engineFuelPort"))) (to (node (document "d0") (qualified-name "Port Example::FuelInPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelReturn"))) (to (node (document "d0") (qualified-name "Port Example::Fuel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelInPort::fuelSupply"))) (to (node (document "d0") (qualified-name "Port Example::Fuel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelInPort::temperature"))) (to (node (document "d0") (qualified-name "Port Example::Temp"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelReturn"))) (to (node (document "d0") (qualified-name "Port Example::Fuel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelOutPort::fuelSupply"))) (to (node (document "d0") (qualified-name "Port Example::Fuel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelOutPort::temperature"))) (to (node (document "d0") (qualified-name "Port Example::Temp"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Port Example::FuelTankAssembly::fuelTankPort"))) (to (node (document "d0") (qualified-name "Port Example::FuelOutPort"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
