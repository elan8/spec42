# META
~~~ini
description=KerML Mass Roll-up: Vehicles_1
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_1 {
	private import ScalarValues::String;
	private import MassRollup_1::*;

	class Vehicle specializes MassedThing {
		feature vin: String;
		feature m redefines mass;
	
		composite engine: Engine subsets subcomponents;
		composite transmission: Transmission subsets subcomponents;
	}
	
	class Engine specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	class Transmission specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	// Example usage
	
	private import SI::*;
	feature v: Vehicle {
		feature m redefines Vehicle::m = 1000;
		composite engine redefines Vehicle::engine {
			feature m redefines Engine::m = 100;
		}
		composite transmission redefines Vehicle::transmission {
			feature m redefines Transmission::m = 50;
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
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
LineComment,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
LineComment,
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
  (package_def 'Vehicles_1'
    (import_decl private 'ScalarValues::String')
    (import_decl private 'MassRollup_1::*')
    (class_def 'Vehicle' :> 'MassedThing'
      (feature_def 'vin' : 'String')
      (feature_def 'm' :>> 'mass')
      (feature_def composite 'engine' : 'Engine' :> 'subcomponents')
      (feature_def composite 'transmission' : 'Transmission' :> 'subcomponents'))
    (class_def 'Engine' :> 'MassedThing'
      (feature_def 'serialNumber' : 'String')
      (feature_def 'm' :>> 'mass')
      (line_comment))
    (class_def 'Transmission' :> 'MassedThing'
      (feature_def 'serialNumber' : 'String')
      (feature_def 'm' :>> 'mass')
      (line_comment))
    (line_comment)
    (import_decl private 'SI::*')
    (feature_def 'v' : 'Vehicle'
      (feature_def 'm' :>> 'Vehicle::m' value)
      (feature_def composite 'engine' :>> 'Vehicle::engine'
        (feature_def 'm' :>> 'Engine::m' value))
      (feature_def composite 'transmission' :>> 'Vehicle::transmission'
        (feature_def 'm' :>> 'Transmission::m' value)))
    (line_comment)))
~~~
# FORMAT
~~~sysml
package Vehicles_1 {
    private import ScalarValues::String;
    private import MassRollup_1::*;

    class Vehicle specializes MassedThing {
        feature vin : String;
        feature m redefines mass;

        composite engine: Engine subsets subcomponents;
        composite transmission: Transmission subsets subcomponents;
    }

    class Engine specializes MassedThing {
        feature serialNumber : String;
        feature m redefines mass;

        // ...
    }

    class Transmission specializes MassedThing {
        feature serialNumber : String;
        feature m redefines mass;

        // ...
    }

    // Example usage

    private import SI::*;
    feature v : Vehicle {
        feature m redefines Vehicle::m = 1000;
        composite engine redefines Vehicle::engine {
            feature m redefines Engine::m = 100;
        }
        composite transmission redefines Vehicle::transmission {
            feature m redefines Transmission::m = 50;
        }
    }

    // v.totalMass evaluates to 1150.0
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Vehicles_1'
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (namespace_import private -> 'MassRollup_1'[unresolved])
      (class_def 'Vehicle' :> 'MassedThing'[unresolved]
        (feature_def 'vin' : 'String'[unresolved])
        (feature_def 'm' :>> 'mass'[unresolved])
        (feature_def composite 'engine' : 'Vehicles_1::Engine'[class_def] :> 'subcomponents'[unresolved])
        (feature_def composite 'transmission' : 'Vehicles_1::Transmission'[class_def] :> 'subcomponents'[unresolved]))
      (class_def 'Engine' :> 'MassedThing'[unresolved]
        (feature_def 'serialNumber' : 'String'[unresolved])
        (feature_def 'm' :>> 'mass'[unresolved]))
      (class_def 'Transmission' :> 'MassedThing'[unresolved]
        (feature_def 'serialNumber' : 'String'[unresolved])
        (feature_def 'm' :>> 'mass'[unresolved]))
      (namespace_import private -> 'SI'[unresolved])
      (feature_def 'v' : 'Vehicles_1::Vehicle'[class_def]
        (feature_def 'm' :>> 'Vehicles_1::Vehicle::m'[feature_def]
          (feature_value (=)))
        (feature_def composite 'engine' :>> 'Vehicles_1::Vehicle::engine'[feature_def]
          (feature_def 'm' :>> 'Vehicles_1::Engine::m'[feature_def]
            (feature_value (=))))
        (feature_def composite 'transmission' :>> 'Vehicles_1::Vehicle::transmission'[feature_def]
          (feature_def 'm' :>> 'Vehicles_1::Transmission::m'[feature_def]
            (feature_value (=))))))))
~~~
