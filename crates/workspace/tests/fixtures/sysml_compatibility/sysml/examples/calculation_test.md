# META
~~~ini
description=SysML Example (Simple Tests): CalculationTest
type=file
~~~
# SOURCE
~~~sysml
package CalculationExample {
	private import ISQ::*;
	private import NumericalFunctions::*;
	
	part def VehiclePart {
		attribute m : MassValue;
	}
	
	part def Vehicle :> VehiclePart;
	
	part vehicle : Vehicle {		
		part eng : VehiclePart;		
		part trans : VehiclePart;
		attribute ::> m = ms.totalMass;
	}
	
	calc def MassSum {
		in partMasses : MassValue[0..*];
		return totalMass : MassValue = sum(partMasses);
	}
	
	calc ms: MassSum {
		in partMasses = (vehicle.eng.m, vehicle.trans.m);
		return totalMass;
	}
	
	part vehicles[*] = (vehicle, vehicle);
	attribute masses1[*] = (vehicles as VehiclePart).m;
	attribute masses2[*] = (vehicles as vehicle).m;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonColonGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,OpenParen,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,CloseParen,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
KwAttribute,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CalculationExample'
    (import_decl private 'ISQ::*')
    (import_decl private 'NumericalFunctions::*')
    (part_def 'VehiclePart'
      (attribute_usage 'm' : 'MassValue'))
    (part_def 'Vehicle' :> 'VehiclePart')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'eng' : 'VehiclePart')
      (part_usage 'trans' : 'VehiclePart')
      (attribute_usage references 'm' value))
    (calc_def 'MassSum'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (return_member))
    (calc_usage 'ms' : 'MassSum'
      (default_ref_usage in 'partMasses' value)
      (return_member))
    (part_usage 'vehicles' multiplicity value)
    (attribute_usage 'masses1' multiplicity value)
    (attribute_usage 'masses2' multiplicity value)))
~~~
# FORMAT
~~~sysml
package CalculationExample {
    private import ISQ::*;
    private import NumericalFunctions::*;

    part def VehiclePart {
        attribute m : MassValue;
    }

    part def Vehicle :> VehiclePart;

    part vehicle : Vehicle {
        part eng : VehiclePart;
        part trans : VehiclePart;
        attribute ::> m = ms.totalMass;
    }

    calc def MassSum {
        in partMasses : MassValue [0..*];
        return totalMass : MassValue = sum(partMasses);
    }

    calc ms : MassSum {
        in partMasses = (vehicle.eng.m, vehicle.trans.m);
        return totalMass;
    }

    part vehicles [*] = (vehicle, vehicle);
    attribute masses1 [*] = (vehicles as VehiclePart).m;
    attribute masses2 [*] = (vehicles as vehicle).m;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# SMG
~~~
(model
  (namespace
    (package 'CalculationExample'
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (part_def 'VehiclePart'
        (attribute_usage composite 'm' : 'MassValue'[unresolved]))
      (part_def 'Vehicle' :> 'CalculationExample::VehiclePart'[part_def])
      (part_usage 'vehicle' : 'CalculationExample::Vehicle'[part_def]
        (part_usage composite 'eng' : 'CalculationExample::VehiclePart'[part_def])
        (part_usage composite 'trans' : 'CalculationExample::VehiclePart'[part_def])
        (attribute_usage composite :> 'CalculationExample::VehiclePart::m'[attribute_usage]
          (feature_value (=))))
      (calculation_def 'MassSum'
        (reference_usage in reference 'partMasses' : 'MassValue'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out 'totalMass' : 'MassValue'[unresolved]
            (feature_value (=)))))
      (calculation_usage 'ms' : 'CalculationExample::MassSum'[calculation_def]
        (reference_usage in reference 'partMasses'
          (feature_value (=)))
        (return_parameter_membership
          (feature_def out 'totalMass')))
      (part_usage 'vehicles'
        (multiplicity_range [*])
        (feature_value (=)))
      (attribute_usage 'masses1'
        (multiplicity_range [*])
        (feature_value (=)))
      (attribute_usage 'masses2'
        (multiplicity_range [*])
        (feature_value (=))))))
~~~
