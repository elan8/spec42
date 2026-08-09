# META
~~~ini
description=SysML Validation (07-Variant Configuration): 7a-Variant Configuration - General Concept
type=file
~~~
# SOURCE
~~~sysml
package '7a-Variant Configuration - General Concept' {
	
	part def Vehicle;
	
	part part1;
	part part2;
	part part3;
	part part4;
	part part5;
	part part6;
	
	abstract part anyVehicleConfig : Vehicle {
		
		variation part subsystemA {
			variant part subsystem1 {
				part :>> part1;
				part :>> part2;
			}
			variant part subsystem2 {
				part :>> part2;
				part :>> part3;
			}
		}

		variation part subsystemB {
			variant part subsystem3 {
				part :>> part4;
				part :>> part5;
			}
			variant part subsystem4 {
				part :>> part5;
				part :>> part6;
			}
		}
		
		assert constraint {
			subsystemA != subsystemA::subsystem2 | 
			subsystemB == subsystemB::subsystem3
		}
		
	}
	
	part vehicleConfigA :> anyVehicleConfig {		
		part :>> subsystemA = subsystemA::subsystem1;
		part :>> subsystemB = subsystemB::subsystem3;
	}
	
	part VehicleConfigB :> anyVehicleConfig {
		part :>> subsystemA = subsystemA::subsystem2;
		part :>> subsystemB = subsystemB::subsystem3;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwAbstract,KwPart,Ident,Colon,Ident,OpenCurly,
KwVariation,KwPart,Ident,OpenCurly,
KwVariant,KwPart,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwVariant,KwPart,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwVariation,KwPart,Ident,OpenCurly,
KwVariant,KwPart,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwVariant,KwPart,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
Ident,BangEq,Ident,ColonColon,Ident,Pipe,
Ident,EqEq,Ident,ColonColon,Ident,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''7a-Variant Configuration - General Concept''
    (part_def 'Vehicle')
    (part_usage 'part1')
    (part_usage 'part2')
    (part_usage 'part3')
    (part_usage 'part4')
    (part_usage 'part5')
    (part_usage 'part6')
    (part_usage abstract 'anyVehicleConfig' : 'Vehicle'
      (part_usage variation 'subsystemA'
        (variant_usage
          (part_usage 'subsystem1'
            (part_usage :>> 'part1')
            (part_usage :>> 'part2')))
        (variant_usage
          (part_usage 'subsystem2'
            (part_usage :>> 'part2')
            (part_usage :>> 'part3'))))
      (part_usage variation 'subsystemB'
        (variant_usage
          (part_usage 'subsystem3'
            (part_usage :>> 'part4')
            (part_usage :>> 'part5')))
        (variant_usage
          (part_usage 'subsystem4'
            (part_usage :>> 'part5')
            (part_usage :>> 'part6'))))
      (sysml_decl
        (result_expr_member)))
    (part_usage 'vehicleConfigA' :> 'anyVehicleConfig'
      (part_usage :>> 'subsystemA' value)
      (part_usage :>> 'subsystemB' value))
    (part_usage 'VehicleConfigB' :> 'anyVehicleConfig'
      (part_usage :>> 'subsystemA' value)
      (part_usage :>> 'subsystemB' value))))
~~~
# FORMAT
~~~sysml
package '7a-Variant Configuration - General Concept' {
    part def Vehicle;

    part part1;
    part part2;
    part part3;
    part part4;
    part part5;
    part part6;

    abstract part anyVehicleConfig : Vehicle {
        variation part subsystemA {
            variant part subsystem1 {
				part :>> part1;
				part :>> part2;
			}
            variant part subsystem2 {
				part :>> part2;
				part :>> part3;
			}
        }

        variation part subsystemB {
            variant part subsystem3 {
				part :>> part4;
				part :>> part5;
			}
            variant part subsystem4 {
				part :>> part5;
				part :>> part6;
			}
        }

        assert constraint {
            = subsystemA != subsystemA::subsystem2 | subsystemB == subsystemB::subsystem3;
        }
    }

    part vehicleConfigA :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem1;
        part :>> subsystemB = subsystemB::subsystem3;
    }

    part VehicleConfigB :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem2;
        part :>> subsystemB = subsystemB::subsystem3;
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package '7a-Variant Configuration - General Concept'
      (part_def 'Vehicle')
      (part_usage 'part1')
      (part_usage 'part2')
      (part_usage 'part3')
      (part_usage 'part4')
      (part_usage 'part5')
      (part_usage 'part6')
      (part_usage abstract 'anyVehicleConfig' : '7a-Variant Configuration - General Concept::Vehicle'[part_def]
        (part_usage variation composite 'subsystemA'
          (variant_usage
            (part_usage composite 'subsystem1'
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part1'[part_usage])
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part2'[part_usage])))
          (variant_usage
            (part_usage composite 'subsystem2'
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part2'[part_usage])
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part3'[part_usage]))))
        (part_usage variation composite 'subsystemB'
          (variant_usage
            (part_usage composite 'subsystem3'
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part4'[part_usage])
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part5'[part_usage])))
          (variant_usage
            (part_usage composite 'subsystem4'
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part5'[part_usage])
              (part_usage composite :>> '7a-Variant Configuration - General Concept::part6'[part_usage]))))
        (assert_constraint_usage
          (result_expr_membership)))
      (part_usage 'vehicleConfigA' :> '7a-Variant Configuration - General Concept::anyVehicleConfig'[part_usage]
        (part_usage composite :>> '7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA'[part_usage]
          (feature_value (=)))
        (part_usage composite :>> '7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB'[part_usage]
          (feature_value (=))))
      (part_usage 'VehicleConfigB' :> '7a-Variant Configuration - General Concept::anyVehicleConfig'[part_usage]
        (part_usage composite :>> '7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA'[part_usage]
          (feature_value (=)))
        (part_usage composite :>> '7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB'[part_usage]
          (feature_value (=)))))))
~~~
