# META
~~~ini
description=SysML Training 11 (Interfaces): Interface Example
type=file
~~~
# SOURCE
~~~sysml
package 'Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
	}
	
	part vehicle : Vehicle {	
		part tankAssy : FuelTankAssembly;		
		part eng : Engine;
		
		interface : FuelInterface connect 
			supplierPort ::> tankAssy.fuelTankPort to 
			consumerPort ::> eng.engineFuelPort;
	} 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwInterface,Colon,Ident,KwConnect,
Ident,ColonColonGt,Ident,Dot,Ident,KwTo,
Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Interface Example''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (interface_def 'FuelInterface'
      (interface_end end 'supplierPort' : 'FuelOutPort')
      (interface_end end 'consumerPort' : 'FuelInPort'))
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tankAssy' : 'FuelTankAssembly')
      (part_usage 'eng' : 'Engine')
      (interface_usage 'FuelInterface'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Interface Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    interface def FuelInterface {
        end supplierPort : FuelOutPort;
        end consumerPort : FuelInPort;
    }

    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;

        interface : FuelInterface connect supplierPort ::> tankAssy.fuelTankPort to consumerPort ::> eng.engineFuelPort;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'tankAssy::fuelTankPort'
semantic.unresolved_name 'eng::engineFuelPort'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'tankAssy::fuelTankPort'
semantic.unresolved_name 'eng::engineFuelPort'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Interface Example'
      (namespace_import private -> 'Port Example'[unresolved])
      (part_def 'Vehicle')
      (interface_def 'FuelInterface'
        (port_usage end 'supplierPort' : 'FuelOutPort'[unresolved])
        (port_usage end 'consumerPort' : 'FuelInPort'[unresolved]))
      (part_usage 'vehicle' : 'Interface Example::Vehicle'[part_def]
        (part_usage composite 'tankAssy' : 'FuelTankAssembly'[unresolved])
        (part_usage composite 'eng' : 'Engine'[unresolved])
        (interface_usage composite : 'Interface Example::FuelInterface'[interface_def]
          (connector_end 'supplierPort' :> 'tankAssy::fuelTankPort'[unresolved])
          (connector_end 'consumerPort' :> 'eng::engineFuelPort'[unresolved]))))))
~~~
