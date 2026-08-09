# META
~~~ini
description=KerML Mass Roll-up: Vehicles_2
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_2 {
	private import ScalarValues::String;
	private import MassRollup_1::*;
	
	class CarPart specializes MassedThing {		
		feature serialNumber: String;
		feature m redefines mass;
		
		composite subparts: CarPart[0..*] redefines subcomponents;
	}
	
	feature vehicle: CarPart {	
		feature vin redefines serialNumber;
		
		composite engine: CarPart subsets subparts {
			//...
		}
		
		composite transmission: CarPart subsets subparts {
			//...
		}
	}
	
	// Example usage
	
	private import SI::*;
	feature v: vehicle {
		feature m redefines CarPart::m = 1000;
		composite engine redefines vehicle::engine {
			feature m redefines CarPart::m = 100;
		}
		composite transmission redefines vehicle::transmission {
			feature m redefines CarPart::m = 50;
		}
	}
	
	// v.totalMass evaluates to 1150.0
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,OpenCurly,
LineComment,
CloseCurly,
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Vehicles_2'
    (import_decl private 'ScalarValues::String')
    (import_decl private 'MassRollup_1::*')
    (class_def 'CarPart' :> 'MassedThing'
      (feature_def 'serialNumber' : 'String')
      (feature_def 'm' :>> 'mass')
      (feature_def composite 'subparts' : 'CarPart' multiplicity :>> 'subcomponents'))
    (feature_def 'vehicle' : 'CarPart'
      (feature_def 'vin' :>> 'serialNumber')
      (feature_def composite 'engine' : 'CarPart' :> 'subparts'
        (line_comment))
      (feature_def composite 'transmission' : 'CarPart' :> 'subparts'
        (line_comment)))
    (line_comment)
    (import_decl private 'SI::*')
    (feature_def 'v' : 'vehicle'
      (feature_def 'm' :>> 'CarPart::m' value)
      (feature_def composite 'engine' :>> 'vehicle::engine'
        (feature_def 'm' :>> 'CarPart::m' value))
      (feature_def composite 'transmission' :>> 'vehicle::transmission'
        (feature_def 'm' :>> 'CarPart::m' value)))
    (line_comment)))
~~~
# FORMAT
~~~sysml
package Vehicles_2 {
    private import ScalarValues::String;
    private import MassRollup_1::*;

    class CarPart specializes MassedThing {
        feature serialNumber : String;
        feature m redefines mass;

        composite subparts: CarPart [0..*] redefines subcomponents;
    }

    feature vehicle : CarPart {
        feature vin redefines serialNumber;

        composite engine: CarPart subsets subparts {
            //...
        }

        composite transmission: CarPart subsets subparts {
            //...
        }
    }

    // Example usage

    private import SI::*;
    feature v : vehicle {
        feature m redefines CarPart::m = 1000;
        composite engine redefines vehicle::engine {
            feature m redefines CarPart::m = 100;
        }
        composite transmission redefines vehicle::transmission {
            feature m redefines CarPart::m = 50;
        }
    }

    // v.totalMass evaluates to 1150.0
}
~~~
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Vehicles_2'
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (namespace_import private -> 'MassRollup_1'[unresolved])
      (class_def 'CarPart' :> 'MassedThing'[unresolved]
        (feature_def 'serialNumber' : 'String'[unresolved])
        (feature_def 'm' :>> 'mass'[unresolved])
        (feature_def composite 'subparts' : 'Vehicles_2::CarPart'[class_def] :>> 'subcomponents'[unresolved]
          (multiplicity_range [0..*])))
      (feature_def 'vehicle' : 'Vehicles_2::CarPart'[class_def]
        (feature_def 'vin' :>> 'Vehicles_2::CarPart::serialNumber'[feature_def])
        (feature_def composite 'engine' : 'Vehicles_2::CarPart'[class_def] :> 'Vehicles_2::CarPart::subparts'[feature_def])
        (feature_def composite 'transmission' : 'Vehicles_2::CarPart'[class_def] :> 'Vehicles_2::CarPart::subparts'[feature_def]))
      (namespace_import private -> 'SI'[unresolved])
      (feature_def 'v' : 'Vehicles_2::vehicle'[feature_def]
        (feature_def 'm' :>> 'Vehicles_2::CarPart::m'[feature_def]
          (feature_value (=)))
        (feature_def composite 'engine' :>> 'Vehicles_2::vehicle::engine'[feature_def]
          (feature_def 'm' :>> 'Vehicles_2::CarPart::m'[feature_def]
            (feature_value (=))))
        (feature_def composite 'transmission' :>> 'Vehicles_2::vehicle::transmission'[feature_def]
          (feature_def 'm' :>> 'Vehicles_2::CarPart::m'[feature_def]
            (feature_value (=))))))))
~~~
