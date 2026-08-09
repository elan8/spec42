# META
~~~ini
description=SysML Training 12 (Binding Connectors): Binding Connectors Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Binding Connectors Example-1' {
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
			
			bind fuelTankPort.fuelSupply = pump.pumpOut;
			bind fuelTankPort.fuelReturn = tank.fuelIn;
			
			part pump : FuelPump {
				out item pumpOut : Fuel;
				in item pumpIn : Fuel;
			}
			
			part tank : FuelTank {
				out item fuelOut : Fuel;
				in item fuelIn : Fuel;
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
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Binding Connectors Example-1''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (part_def 'FuelPump')
    (part_def 'FuelTank')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tank' : 'FuelTankAssembly'
        (port_usage :>> 'fuelTankPort'
          (item_usage out :>> 'fuelSupply')
          (item_usage in :>> 'fuelReturn'))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (part_usage 'pump' : 'FuelPump'
          (item_usage out 'pumpOut' : 'Fuel')
          (item_usage in 'pumpIn' : 'Fuel'))
        (part_usage 'tank' : 'FuelTank'
          (item_usage out 'fuelOut' : 'Fuel')
          (item_usage in 'fuelIn' : 'Fuel'))))))
~~~
# FORMAT
~~~sysml
package 'Binding Connectors Example-1' {
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

            bind fuelTankPort.fuelSupply = pump.pumpOut;
            bind fuelTankPort.fuelReturn = tank.fuelIn;

            part pump : FuelPump {
                out item pumpOut : Fuel;
                in item pumpIn : Fuel;
            }

            part tank : FuelTank {
                out item fuelOut : Fuel;
                in item fuelIn : Fuel;
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
    (package 'Binding Connectors Example-1'
      (namespace_import private -> 'Port Example'[unresolved])
      (part_def 'Vehicle')
      (part_def 'FuelPump')
      (part_def 'FuelTank')
      (part_usage 'vehicle' : 'Binding Connectors Example-1::Vehicle'[part_def]
        (part_usage composite 'tank' : 'FuelTankAssembly'[unresolved]
          (port_usage composite :>> 'fuelTankPort'[unresolved]
            (item_usage out :>> 'fuelSupply'[unresolved])
            (item_usage in :>> 'fuelReturn'[unresolved]))
          (binding_connector_def
            (connector_end 'fuelTankPort.fuelSupply')
            (connector_end 'pump.pumpOut'))
          (binding_connector_def
            (connector_end 'fuelTankPort.fuelReturn')
            (connector_end 'tank.fuelIn'))
          (part_usage composite 'pump' : 'Binding Connectors Example-1::FuelPump'[part_def]
            (item_usage out 'pumpOut' : 'Fuel'[unresolved])
            (item_usage in 'pumpIn' : 'Fuel'[unresolved]))
          (part_usage composite 'tank' : 'Binding Connectors Example-1::FuelTank'[part_def]
            (item_usage out 'fuelOut' : 'Fuel'[unresolved])
            (item_usage in 'fuelIn' : 'Fuel'[unresolved])))))))
~~~
