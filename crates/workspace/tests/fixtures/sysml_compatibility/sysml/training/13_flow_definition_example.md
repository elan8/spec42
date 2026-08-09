# META
~~~ini
description=SysML Training 13 (Flows): Flow Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Definition Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	flow def FuelFlow {
		ref :>> payload : Fuel;
		end port supplierPort : FuelOutPort;
		end port consumerPort : FuelInPort;
	}
	
	part vehicle : Vehicle {
		part tankAssy : FuelTankAssembly;
		part eng : Engine;
		
		flow : FuelFlow of Fuel
		  from tankAssy.fuelTankPort.fuelSupply
			to eng.engineFuelPort.fuelSupply;
			
	} 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwFlow,KwDef,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwFlow,Colon,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Flow Definition Example''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (flow_def 'FuelFlow'
      (ref_usage ref :>> 'payload' : 'Fuel')
      (interface_end end 'supplierPort' : 'FuelOutPort')
      (interface_end end 'consumerPort' : 'FuelInPort'))
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tankAssy' : 'FuelTankAssembly')
      (part_usage 'eng' : 'Engine')
      (flow_usage 'FuelFlow' : 'Fuel'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Flow Definition Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    flow def FuelFlow {
        ref :>> payload : Fuel;
        end supplierPort : FuelOutPort;
        end consumerPort : FuelInPort;
    }

    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;

        flow : FuelFlow of Fuel from tankAssy.fuelTankPort.fuelSupply to eng.engineFuelPort.fuelSupply;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'payload'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'Fuel'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'payload'
semantic.unresolved_name 'Fuel'
semantic.unresolved_name 'FuelOutPort'
semantic.unresolved_name 'FuelInPort'
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'Fuel'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Flow Definition Example'
      (namespace_import private -> 'Port Example'[unresolved])
      (part_def 'Vehicle')
      (flow_def 'FuelFlow'
        (reference_usage reference :>> 'payload'[unresolved] : 'Fuel'[unresolved])
        (port_usage end 'supplierPort' : 'FuelOutPort'[unresolved])
        (port_usage end 'consumerPort' : 'FuelInPort'[unresolved]))
      (part_usage 'vehicle' : 'Flow Definition Example::Vehicle'[part_def]
        (part_usage composite 'tankAssy' : 'FuelTankAssembly'[unresolved])
        (part_usage composite 'eng' : 'Engine'[unresolved])
        (flow_usage composite : 'Flow Definition Example::FuelFlow'[flow_def] : 'Fuel'[unresolved]
          (connector_end 'tankAssy.fuelTankPort.fuelSupply')
          (connector_end 'eng.engineFuelPort.fuelSupply'))))))
~~~
