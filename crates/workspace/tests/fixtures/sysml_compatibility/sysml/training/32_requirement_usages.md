# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Usages' {
	private import SI::*;
	private import 'Requirement Definitions'::*;
	
	requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 2000[kg];
		
		assume constraint {
			doc /* Full tank is full. */
			vehicle.fuelMass == vehicle.fuelFullMass
		}
	}
	
	requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 1500[kg];
		
		assume constraint {
			doc /* Full tank is empty. */
			vehicle.fuelMass == 0[kg]
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,OpenCurly,
KwDoc,RegularComment,
Ident,Dot,Ident,EqEq,Ident,Dot,Ident,
CloseCurly,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,OpenCurly,
KwDoc,RegularComment,
Ident,Dot,Ident,EqEq,DecimalValue,OpenSquare,Ident,CloseSquare,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Requirement Usages''
    (import_decl private 'SI::*')
    (import_decl private ''Requirement Definitions'::*')
    (requirement_usage 'fullVehicleMassLimit' : 'VehicleMassLimitationRequirement'
      (sysml_decl 'vehicle' : 'Vehicle')
      (attribute_usage :>> 'massReqd' value)
      (sysml_decl
        (documentation)
        (result_expr_member)))
    (requirement_usage 'emptyVehicleMassLimit' : 'VehicleMassLimitationRequirement'
      (sysml_decl 'vehicle' : 'Vehicle')
      (attribute_usage :>> 'massReqd' value)
      (sysml_decl
        (documentation)
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
package 'Requirement Usages' {
    private import SI::*;
    private import 'Requirement Definitions'::*;

    requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        attribute :>> massReqd = 2000[kg];

        assume constraint {
            doc /* Full tank is full. */
            = vehicle.fuelMass == vehicle.fuelFullMass;
        }
    }

    requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        attribute :>> massReqd = 1500[kg];

        assume constraint {
            doc /* Full tank is empty. */
            = vehicle.fuelMass == 0[kg];
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Requirement Usages'
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'Requirement Definitions'[unresolved])
      (requirement_usage 'fullVehicleMassLimit' : 'VehicleMassLimitationRequirement'[unresolved]
        (subject_membership in 'vehicle' : 'Vehicle'[unresolved])
        (attribute_usage composite :>> 'massReqd'[unresolved]
          (feature_value (=)))
        (assume_constraint_usage composite
          (documentation)
          (result_expr_membership)))
      (requirement_usage 'emptyVehicleMassLimit' : 'VehicleMassLimitationRequirement'[unresolved]
        (subject_membership in 'vehicle' : 'Vehicle'[unresolved])
        (attribute_usage composite :>> 'massReqd'[unresolved]
          (feature_value (=)))
        (assume_constraint_usage composite
          (documentation)
          (result_expr_membership))))))
~~~
