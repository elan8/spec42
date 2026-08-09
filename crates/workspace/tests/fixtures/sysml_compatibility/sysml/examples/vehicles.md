# META
~~~ini
description=SysML Example (Mass Roll-up): Vehicles
type=file
~~~
# SOURCE
~~~sysml
package VehicleMasses {
	private import ScalarValues::*;
	private import MassRollup::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin redefines serialNumber;
		
		part carParts: CarPart[*] redefines subcomponents;
		
		part engine :> simpleThing, carParts {
			//...
		}
		
		part transmission :> simpleThing, carParts {
			//...
		}
	}

	// Example usage
	private import SI::*;	
	part c :> car {
		redefines mass = 1000 [kg];
		part redefines engine {
			redefines mass = 100 [kg];
		}
		
		part redefines transmission {
			redefines mass = 50 [kg];
		}	
	}
	
	// c.totalMass --> 1150.0 [kg]
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,KwRedefines,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwRedefines,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
LineComment,
CloseCurly,
KwPart,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,KwRedefines,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleMasses'
    (import_decl private 'ScalarValues::*')
    (import_decl private 'MassRollup::*')
    (part_def 'CarPart' :> 'MassedThing'
      (attribute_usage 'serialNumber' : 'String'))
    (part_usage 'car' : 'CarPart' :> 'compositeThing'
      (attribute_usage 'vin' :>> 'serialNumber')
      (part_usage 'carParts' : 'CarPart' :>> 'subcomponents' multiplicity)
      (part_usage 'engine' :> 'simpleThing', 'carParts'
        (line_comment))
      (part_usage 'transmission' :> 'simpleThing', 'carParts'
        (line_comment)))
    (line_comment)
    (import_decl private 'SI::*')
    (part_usage 'c' :> 'car'
      (default_ref_usage :>> 'mass' value)
      (part_usage :>> 'engine'
        (default_ref_usage :>> 'mass' value))
      (part_usage :>> 'transmission'
        (default_ref_usage :>> 'mass' value)))
    (line_comment)))
~~~
# FORMAT
~~~sysml
package VehicleMasses {
    private import ScalarValues::*;
    private import MassRollup::*;

    part def CarPart :> MassedThing {
        attribute serialNumber : String;
    }

    part car : CarPart :> compositeThing {
        attribute vin redefines serialNumber;

        part carParts : CarPart redefines subcomponents [*];

        part engine :> simpleThing, carParts {
            //...
        }

        part transmission :> simpleThing, carParts {
            //...
        }
    }

    // Example usage
    private import SI::*;
    part c :> car {
         redefines mass = 1000 [kg];
        part redefines engine {
             redefines mass = 100 [kg];
        }

        part redefines transmission {
             redefines mass = 50 [kg];
        }
    }

    // c.totalMass --> 1150.0 [kg]
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
~~~
# SMG
~~~
(model
  (namespace
    (package 'VehicleMasses'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'MassRollup'[unresolved])
      (part_def 'CarPart' :> 'MassedThing'[unresolved]
        (attribute_usage composite 'serialNumber' : 'String'[unresolved]))
      (part_usage 'car' : 'VehicleMasses::CarPart'[part_def] :> 'compositeThing'[unresolved]
        (attribute_usage composite 'vin' :>> 'VehicleMasses::CarPart::serialNumber'[attribute_usage])
        (part_usage composite 'carParts' : 'VehicleMasses::CarPart'[part_def] :>> 'subcomponents'[unresolved]
          (multiplicity_range [*]))
        (part_usage composite 'engine' :> 'simpleThing'[unresolved] :> 'VehicleMasses::car::carParts'[part_usage])
        (part_usage composite 'transmission' :> 'simpleThing'[unresolved] :> 'VehicleMasses::car::carParts'[part_usage]))
      (namespace_import private -> 'SI'[unresolved])
      (part_usage 'c' :> 'VehicleMasses::car'[part_usage]
        (reference_usage reference :>> 'mass'[unresolved]
          (feature_value (=)))
        (part_usage composite :>> 'VehicleMasses::car::engine'[part_usage]
          (reference_usage reference :>> 'mass'[unresolved]
            (feature_value (=))))
        (part_usage composite :>> 'VehicleMasses::car::transmission'[part_usage]
          (reference_usage reference :>> 'mass'[unresolved]
            (feature_value (=))))))))
~~~
