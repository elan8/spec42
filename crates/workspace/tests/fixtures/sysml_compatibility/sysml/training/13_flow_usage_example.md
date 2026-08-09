# META
~~~ini
description=SysML Training 13 (Flows): Flow Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flow Usage Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	part vehicle : Vehicle {
		part tankAssy : FuelTankAssembly;
		part eng : Engine;
		
		flow of Fuel
		  from tankAssy.fuelTankPort.fuelSupply
			to eng.engineFuelPort.fuelSupply;
			
		flow of Fuel
		  from eng.engineFuelPort.fuelReturn
			to tankAssy.fuelTankPort.fuelReturn;
	} 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwFlow,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,
KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Flow Usage Example''
    (import_decl private ''Port Example'::*')
    (part_def 'Vehicle')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'tankAssy' : 'FuelTankAssembly')
      (part_usage 'eng' : 'Engine')
      (flow_usage 'of')
      (flow_usage 'of'))))
~~~
# FORMAT
~~~sysml
package 'Flow Usage Example' {
    private import 'Port Example'::*;

    part def Vehicle;

    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;

        flow of;

        flow of;
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'FuelTankAssembly'
semantic.unresolved_name 'Engine'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Flow Usage Example'
      (namespace_import private -> 'Port Example'[unresolved])
      (part_def 'Vehicle')
      (part_usage 'vehicle' : 'Flow Usage Example::Vehicle'[part_def]
        (part_usage composite 'tankAssy' : 'FuelTankAssembly'[unresolved])
        (part_usage composite 'eng' : 'Engine'[unresolved])
        (flow_usage composite 'of')
        (flow_usage composite 'of')))))
~~~
