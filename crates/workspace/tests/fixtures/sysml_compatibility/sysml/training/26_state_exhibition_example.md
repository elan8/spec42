# META
~~~ini
description=SysML Training 26 (State Exhibition): State Exhibition Example
type=file
~~~
# SOURCE
~~~sysml
package 'State Exhibition Example' {
	private import 'Transition Actions'::*;
	
	part vehicle : Vehicle {
		
		part vehicleController : VehicleController;
		
		exhibit vehicleStates {
			in operatingVehicle = vehicle;
			in controller = vehicleController;
		}

	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwExhibit,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''State Exhibition Example''
    (import_decl private ''Transition Actions'::*')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'vehicleController' : 'VehicleController')
      (exhibit_state 'vehicleStates'
        (default_ref_usage in 'operatingVehicle' value)
        (default_ref_usage in 'controller' value)))))
~~~
# FORMAT
~~~sysml
package 'State Exhibition Example' {
    private import 'Transition Actions'::*;

    part vehicle : Vehicle {
        part vehicleController : VehicleController;

        exhibit vehicleStates {
			in operatingVehicle = vehicle;
			in controller = vehicleController;
		}
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'VehicleController'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'VehicleController'
~~~
# SMG
~~~
(model
  (namespace
    (package 'State Exhibition Example'
      (namespace_import private -> 'Transition Actions'[unresolved])
      (part_usage 'vehicle' : 'Vehicle'[unresolved]
        (part_usage composite 'vehicleController' : 'VehicleController'[unresolved])
        (state_usage composite 'vehicleStates'
          (reference_usage in reference 'operatingVehicle'
            (feature_value (=)))
          (reference_usage in reference 'controller'
            (feature_value (=))))))))
~~~
