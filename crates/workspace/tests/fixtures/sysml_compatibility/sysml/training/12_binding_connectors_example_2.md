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
(model
  (namespace
    (package 'Binding Connectors Example-2'
      (namespace_import private -> 'Port Example'[unresolved])
      (part_def 'Vehicle')
      (part_def 'FuelPump')
      (part_def 'FuelTank')
      (part_usage 'vehicle' : 'Binding Connectors Example-2::Vehicle'[part_def]
        (part_usage composite 'tank' : 'FuelTankAssembly'[unresolved]
          (port_usage composite :>> 'fuelTankPort'[unresolved]
            (item_usage out :>> 'fuelSupply'[unresolved])
            (item_usage in :>> 'fuelReturn'[unresolved]))
          (part_usage composite 'pump' : 'Binding Connectors Example-2::FuelPump'[part_def]
            (item_usage out 'pumpOut' : 'Fuel'[unresolved]
              (feature_value (=)))
            (item_usage in 'pumpIn' : 'Fuel'[unresolved]))
          (part_usage composite 'tank' : 'Binding Connectors Example-2::FuelTank'[part_def]
            (item_usage out 'fuelOut' : 'Fuel'[unresolved])
            (item_usage in 'fuelIn' : 'Fuel'[unresolved]
              (feature_value (=)))))))))
~~~
