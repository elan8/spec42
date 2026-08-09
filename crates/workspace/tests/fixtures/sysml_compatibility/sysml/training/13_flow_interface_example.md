# META
~~~ini
description=SysML Training 13 (Flows): Flow Interface Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
		
		flow supplierPort.fuelSupply to consumerPort.fuelSupply;			
		flow consumerPort.fuelReturn to supplierPort.fuelReturn;
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
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
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
  (package_def ''Flow Interface Example''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (interface_def 'FuelInterface'
      (interface_end end 'supplierPort' : 'FuelOutPort')
      (interface_end end 'consumerPort' : 'FuelInPort')
      (flow_usage 'supplierPort')
      (flow_usage 'consumerPort'))
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tankAssy' : 'FuelTankAssembly')
      (part_usage 'eng' : 'Engine')
      (interface_usage 'FuelInterface'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Flow Interface Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    interface def FuelInterface {
        end supplierPort : FuelOutPort;
        end consumerPort : FuelInPort;

        flow supplierPort;
        flow consumerPort;
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
semantic.duplicate_name 'supplierPort'
semantic.duplicate_name 'consumerPort'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'tankAssy::fuelTankPort'
semantic.unresolved_name 'eng::engineFuelPort'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'supplierPort'
semantic.duplicate_name 'consumerPort'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
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
    (package 'Flow Interface Example'
      (namespace_import private -> 'Port Example'[unresolved])
      (part_def 'Vehicle')
      (interface_def 'FuelInterface'
        (port_usage end 'supplierPort' : 'FuelOutPort'[unresolved])
        (port_usage end 'consumerPort' : 'FuelInPort'[unresolved])
        (flow_usage composite 'supplierPort')
        (flow_usage composite 'consumerPort'))
      (part_usage 'vehicle' : 'Flow Interface Example::Vehicle'[part_def]
        (part_usage composite 'tankAssy' : 'FuelTankAssembly'[unresolved])
        (part_usage composite 'eng' : 'Engine'[unresolved])
        (interface_usage composite : 'Flow Interface Example::FuelInterface'[interface_def]
          (connector_end 'supplierPort' :> 'tankAssy::fuelTankPort'[unresolved])
          (connector_end 'consumerPort' :> 'eng::engineFuelPort'[unresolved]))))))
~~~
