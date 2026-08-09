# META
~~~ini
description=SysML Training 12 (Binding Connectors): Binding Connectors Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Binding Connectors Example-2' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	part def FuelPump;
	part def FuelTank;
	
	part vehicle : Vehicle {	
		part tank : FuelTankAssembly {
			port redefines fuelTankPort {
				out item redefines fuelSupply;
				in item redefines fuelReturn;
			}
			
			part pump : FuelPump {
				out item pumpOut : Fuel = fuelTankPort.fuelSupply;
				in item pumpIn : Fuel;
			}
			
			part tank : FuelTank {
				out item fuelOut : Fuel;
				in item fuelIn : Fuel = fuelTankPort.fuelReturn;
			}
		}
	} 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,KwRedefines,Ident,OpenCurly,
KwOut,KwItem,KwRedefines,Ident,Semicolon,
KwIn,KwItem,KwRedefines,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Binding Connectors Example-2''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (part_def 'FuelPump')
    (part_def 'FuelTank')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tank' : 'FuelTankAssembly'
        (port_usage :>> 'fuelTankPort'
          (item_usage out :>> 'fuelSupply')
          (item_usage in :>> 'fuelReturn'))
        (part_usage 'pump' : 'FuelPump'
          (item_usage out 'pumpOut' : 'Fuel' value)
          (item_usage in 'pumpIn' : 'Fuel'))
        (part_usage 'tank' : 'FuelTank'
          (item_usage out 'fuelOut' : 'Fuel')
          (item_usage in 'fuelIn' : 'Fuel' value))))))
~~~
# FORMAT
~~~sysml
package 'Binding Connectors Example-2' {
    private import 'Port Example'::*;

    part def Vehicle;
    part def FuelPump;
    part def FuelTank;

    part vehicle : Vehicle {
        part tank : FuelTankAssembly {
            port redefines fuelTankPort {
                out item redefines fuelSupply;
                in item redefines fuelReturn;
            }

            part pump : FuelPump {
                out item pumpOut : Fuel = fuelTankPort.fuelSupply;
                in item pumpIn : Fuel;
            }

            part tank : FuelTank {
                out item fuelOut : Fuel;
                in item fuelIn : Fuel = fuelTankPort.fuelReturn;
            }
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'fuelTankPort'
semantic.unresolved_name 'fuelSupply'
semantic.unresolved_name 'fuelReturn'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'fuelTankPort'
semantic.unresolved_name 'fuelSupply'
semantic.unresolved_name 'fuelReturn'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'Fuel'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Binding Connectors Example-2"))) (name "Binding Connectors Example-2") (declared-name "Binding Connectors Example-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelPump"))) (name "FuelPump") (declared-name "FuelPump") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelTank"))) (name "FuelTank") (declared-name "FuelTank") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank"))) (name "tank") (declared-name "tank") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle"))))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::fuelTankPort"))) (name "fuelTankPort") (declared-name "fuelTankPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (name "pump") (declared-name "pump") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (name "tank") (declared-name "tank") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle"))) (to (node (document "d0") (qualified-name "Binding Connectors Example-2::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::pump"))) (to (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelPump"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Binding Connectors Example-2::vehicle::tank::tank"))) (to (node (document "d0") (qualified-name "Binding Connectors Example-2::FuelTank"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
