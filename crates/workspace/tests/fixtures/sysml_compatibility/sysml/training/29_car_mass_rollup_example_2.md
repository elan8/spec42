# META
~~~ini
description=SysML Training 29 (Expressions): Car Mass Rollup Example 2
type=file
~~~
# SOURCE
~~~sysml
package 'Car Mass Rollup 1' {
	private import ScalarValues::*;
	private import MassRollup2::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin :>> serialNumber;
		
		part carParts: CarPart[*] :>> subcomponents;
		
		part engine :> carParts {
			//...
		}
		
		part transmission :> carParts {
			//...
		}
	}

	// Example usage
	
	private import SI::kg;
	part c :> car {
		attribute :>> simpleMass = 1000[kg];
		part :>> engine {
			attribute :>> simpleMass = 100[kg];
		}
		
		part redefines transmission {
			attribute :>> simpleMass = 50[kg];
		}	
	}
	
	// c::totalMass --> 1150.0[kg]
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,ColonGtGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
LineComment,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Car Mass Rollup 1''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'MassRollup2::*')
    (part_def 'CarPart' :> 'MassedThing'
      (attribute_usage 'serialNumber' : 'String'))
    (part_usage 'car' : 'CarPart' :> 'compositeThing'
      (attribute_usage 'vin' :>> 'serialNumber')
      (part_usage 'carParts' : 'CarPart' :>> 'subcomponents' multiplicity)
      (part_usage 'engine' :> 'carParts'
        (line_comment))
      (part_usage 'transmission' :> 'carParts'
        (line_comment)))
    (line_comment)
    (import_decl private 'SI::kg')
    (part_usage 'c' :> 'car'
      (attribute_usage :>> 'simpleMass' value)
      (part_usage :>> 'engine'
        (attribute_usage :>> 'simpleMass' value))
      (part_usage :>> 'transmission'
        (attribute_usage :>> 'simpleMass' value)))
    (line_comment)))
~~~
# FORMAT
~~~sysml
package 'Car Mass Rollup 1' {
    private import ScalarValues::*;
    private import MassRollup2::*;

    part def CarPart :> MassedThing {
        attribute serialNumber : String;
    }

    part car : CarPart :> compositeThing {
        attribute vin :>> serialNumber;

        part carParts : CarPart :>> subcomponents [*];

        part engine :> carParts {
            //...
        }

        part transmission :> carParts {
            //...
        }
    }

    // Example usage

    private import SI::kg;
    part c :> car {
        attribute :>> simpleMass = 1000[kg];
        part :>> engine {
            attribute :>> simpleMass = 100[kg];
        }

        part redefines transmission {
            attribute :>> simpleMass = 50[kg];
        }
    }

    // c::totalMass --> 1150.0[kg]
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Car Mass Rollup 1'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'MassRollup2'[unresolved])
      (part_def 'CarPart' :> 'MassedThing'[unresolved]
        (attribute_usage composite 'serialNumber' : 'String'[unresolved]))
      (part_usage 'car' : 'Car Mass Rollup 1::CarPart'[part_def] :> 'compositeThing'[unresolved]
        (attribute_usage composite 'vin' :>> 'Car Mass Rollup 1::CarPart::serialNumber'[attribute_usage])
        (part_usage composite 'carParts' : 'Car Mass Rollup 1::CarPart'[part_def] :>> 'subcomponents'[unresolved]
          (multiplicity_range [*]))
        (part_usage composite 'engine' :> 'Car Mass Rollup 1::car::carParts'[part_usage])
        (part_usage composite 'transmission' :> 'Car Mass Rollup 1::car::carParts'[part_usage]))
      (membership_import private -> 'SI::kg'[unresolved])
      (part_usage 'c' :> 'Car Mass Rollup 1::car'[part_usage]
        (attribute_usage composite :>> 'simpleMass'[unresolved]
          (feature_value (=)))
        (part_usage composite :>> 'Car Mass Rollup 1::car::engine'[part_usage]
          (attribute_usage composite :>> 'simpleMass'[unresolved]
            (feature_value (=))))
        (part_usage composite :>> 'Car Mass Rollup 1::car::transmission'[part_usage]
          (attribute_usage composite :>> 'simpleMass'[unresolved]
            (feature_value (=))))))))
~~~
