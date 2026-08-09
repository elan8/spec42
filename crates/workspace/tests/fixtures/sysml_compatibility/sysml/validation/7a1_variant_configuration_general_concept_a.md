# META
~~~ini
description=SysML Validation (07-Variant Configuration): 7a1-Variant Configuration - General Concept-a
type=file
~~~
# SOURCE
~~~sysml
package '7a1-Variant Configuration - General Concept-a' {
	
	action doX;
	action doY;
	
	part part1;
	part part2;
	part part3 {
		port p1;
	}
	part part4;
	part part5 {
		port p2;
		variation perform action doXorY {
			variant perform doX;
			variant perform doY;
		}
	}
	part part6;
	
	abstract part def SubsystemA {
		abstract part :>> part3[0..1];
	}
	
	abstract part def SubsystemB {
		abstract part :>> part5[1];		
	}
	
	part anyVehicleConfig {
		
		variation part subsystemA : SubsystemA {
			variant part subsystem1 : SubsystemA {
				part :>> part1[1];
				part :>> part2[1];
			}
			variant part subsystem2 : SubsystemA {
				part :>> part2[1];
				part :>> part3[1];
			}
		}

		variation part subsystemB : SubsystemB {
			variant part subsystem3 : SubsystemB {
				part :>> part4[1];
				part :>> part5[1];
			}
			variant part subsystem4 : SubsystemB {
				part :>> part5[1];
				part :>> part6[1];
			}
		}
		
		connect [0..1] subsystemA.part3.p1 to [1] subsystemB.part5.p2;
		
		assert constraint {
			subsystemA != subsystemA::subsystem2 | 
			subsystemB == subsystemB::subsystem3
		}
		
	}
	
	part vehicleConfigA :> anyVehicleConfig {		
		part :>> subsystemA = subsystemA::subsystem1;
		part :>> subsystemB = subsystemB::subsystem3 {
			part :>> part5 {
				perform action :>> doXorY = doX;
			}
		}
	}
	
	part VehicleConfigB :> anyVehicleConfig {
		part :>> subsystemA = subsystemA::subsystem2;
		part :>> subsystemB = subsystemB::subsystem4 {
			part :>> part5 {
				perform action :>> doXorY = doY;
			}
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAction,Ident,Semicolon,
KwAction,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwPort,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwVariation,KwPerform,KwAction,Ident,OpenCurly,
KwVariant,KwPerform,Ident,Semicolon,
KwVariant,KwPerform,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Semicolon,
KwAbstract,KwPart,KwDef,Ident,OpenCurly,
KwAbstract,KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwPart,KwDef,Ident,OpenCurly,
KwAbstract,KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwVariation,KwPart,Ident,Colon,Ident,OpenCurly,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwVariation,KwPart,Ident,Colon,Ident,OpenCurly,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwConnect,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,
Ident,BangEq,Ident,ColonColon,Ident,Pipe,
Ident,EqEq,Ident,ColonColon,Ident,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPerform,KwAction,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPart,ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPerform,KwAction,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''7a1-Variant Configuration - General Concept-a''
    (action_usage 'doX')
    (action_usage 'doY')
    (part_usage 'part1')
    (part_usage 'part2')
    (part_usage 'part3'
      (port_usage 'p1'))
    (part_usage 'part4')
    (part_usage 'part5'
      (port_usage 'p2')
      (perform_action variation 'doXorY'
        (variant_usage
          (perform_action :>> 'doX'))
        (variant_usage
          (perform_action :>> 'doY'))))
    (part_usage 'part6')
    (part_def abstract 'SubsystemA'
      (part_usage abstract :>> 'part3' multiplicity))
    (part_def abstract 'SubsystemB'
      (part_usage abstract :>> 'part5' multiplicity))
    (part_usage 'anyVehicleConfig'
      (part_usage variation 'subsystemA' : 'SubsystemA'
        (variant_usage
          (part_usage 'subsystem1' : 'SubsystemA'
            (part_usage :>> 'part1' multiplicity)
            (part_usage :>> 'part2' multiplicity)))
        (variant_usage
          (part_usage 'subsystem2' : 'SubsystemA'
            (part_usage :>> 'part2' multiplicity)
            (part_usage :>> 'part3' multiplicity))))
      (part_usage variation 'subsystemB' : 'SubsystemB'
        (variant_usage
          (part_usage 'subsystem3' : 'SubsystemB'
            (part_usage :>> 'part4' multiplicity)
            (part_usage :>> 'part5' multiplicity)))
        (variant_usage
          (part_usage 'subsystem4' : 'SubsystemB'
            (part_usage :>> 'part5' multiplicity)
            (part_usage :>> 'part6' multiplicity))))
      (connection_usage
        (connector_end)
        (connector_end))
      (sysml_decl
        (result_expr_member)))
    (part_usage 'vehicleConfigA' :> 'anyVehicleConfig'
      (part_usage :>> 'subsystemA' value)
      (part_usage :>> 'subsystemB' value
        (part_usage :>> 'part5'
          (malformed))))
    (part_usage 'VehicleConfigB' :> 'anyVehicleConfig'
      (part_usage :>> 'subsystemA' value)
      (part_usage :>> 'subsystemB' value
        (part_usage :>> 'part5'
          (malformed))))))
~~~
# FORMAT
~~~sysml
package '7a1-Variant Configuration - General Concept-a' {
    action doX;
    action doY;

    part part1;
    part part2;
    part part3 {
        port p1;
    }
    part part4;
    part part5 {
        port p2;
        variation perform action doXorY {
            variant perform doX;
            variant perform doY;
        }
    }
    part part6;

    abstract part def SubsystemA {
        abstract part :>> part3 [0..1];
    }

    abstract part def SubsystemB {
        abstract part :>> part5 [1];
    }

    part anyVehicleConfig {
        variation part subsystemA : SubsystemA {
            variant part subsystem1 : SubsystemA {
				part :>> part1[1];
				part :>> part2[1];
			}
            variant part subsystem2 : SubsystemA {
				part :>> part2[1];
				part :>> part3[1];
			}
        }

        variation part subsystemB : SubsystemB {
            variant part subsystem3 : SubsystemB {
				part :>> part4[1];
				part :>> part5[1];
			}
            variant part subsystem4 : SubsystemB {
				part :>> part5[1];
				part :>> part6[1];
			}
        }

        connect [0..1] subsystemA.part3.p1 to [1] subsystemB.part5.p2;

        assert constraint {
            = subsystemA != subsystemA::subsystem2 | subsystemB == subsystemB::subsystem3;
        }
    }

    part vehicleConfigA :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem1;
        part :>> subsystemB = subsystemB::subsystem3 {
            part :>> part5 {
                = doX;
            }
        }
    }

    part VehicleConfigB :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem2;
        part :>> subsystemB = subsystemB::subsystem4 {
            part :>> part5 {
                = doY;
            }
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
~~~
# SMG
~~~
(model
  (namespace
    (package '7a1-Variant Configuration - General Concept-a'
      (action_usage 'doX')
      (action_usage 'doY')
      (part_usage 'part1')
      (part_usage 'part2')
      (part_usage 'part3'
        (port_usage composite 'p1'))
      (part_usage 'part4')
      (part_usage 'part5'
        (port_usage composite 'p2')
        (perform_action_usage variation 'doXorY'
          (variant_usage
            (perform_action_usage :>> '7a1-Variant Configuration - General Concept-a::doX'[action_usage]))
          (variant_usage
            (perform_action_usage :>> '7a1-Variant Configuration - General Concept-a::doY'[action_usage]))))
      (part_usage 'part6')
      (part_def abstract 'SubsystemA'
        (part_usage abstract composite :>> '7a1-Variant Configuration - General Concept-a::part3'[part_usage]
          (multiplicity_range [0..1])))
      (part_def abstract 'SubsystemB'
        (part_usage abstract composite :>> '7a1-Variant Configuration - General Concept-a::part5'[part_usage]
          (multiplicity_range [1])))
      (part_usage 'anyVehicleConfig'
        (part_usage variation composite 'subsystemA' : '7a1-Variant Configuration - General Concept-a::SubsystemA'[part_def]
          (variant_usage
            (part_usage composite 'subsystem1' : '7a1-Variant Configuration - General Concept-a::SubsystemA'[part_def]
              (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::part1'[part_usage]
                (multiplicity_range [1]))
              (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::part2'[part_usage]
                (multiplicity_range [1]))))
          (variant_usage
            (part_usage composite 'subsystem2' : '7a1-Variant Configuration - General Concept-a::SubsystemA'[part_def]
              (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::part2'[part_usage]
                (multiplicity_range [1]))
              (part_usage composite :>> ''[part_usage]
                (multiplicity_range [1])))))
        (part_usage variation composite 'subsystemB' : '7a1-Variant Configuration - General Concept-a::SubsystemB'[part_def]
          (variant_usage
            (part_usage composite 'subsystem3' : '7a1-Variant Configuration - General Concept-a::SubsystemB'[part_def]
              (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::part4'[part_usage]
                (multiplicity_range [1]))
              (part_usage composite :>> ''[part_usage]
                (multiplicity_range [1]))))
          (variant_usage
            (part_usage composite 'subsystem4' : '7a1-Variant Configuration - General Concept-a::SubsystemB'[part_def]
              (part_usage composite :>> ''[part_usage]
                (multiplicity_range [1]))
              (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::part6'[part_usage]
                (multiplicity_range [1])))))
        (connection_usage composite
          (connector_end 'subsystemA.part3.p1')
          (connector_end 'subsystemB.part5.p2'))
        (assert_constraint_usage
          (result_expr_membership)))
      (part_usage 'vehicleConfigA' :> '7a1-Variant Configuration - General Concept-a::anyVehicleConfig'[part_usage]
        (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA'[part_usage]
          (feature_value (=)))
        (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB'[part_usage]
          (feature_value (=))
          (part_usage composite :>> ''[part_usage]
            (not_implemented 'malformed'))))
      (part_usage 'VehicleConfigB' :> '7a1-Variant Configuration - General Concept-a::anyVehicleConfig'[part_usage]
        (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA'[part_usage]
          (feature_value (=)))
        (part_usage composite :>> '7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB'[part_usage]
          (feature_value (=))
          (part_usage composite :>> ''[part_usage]
            (not_implemented 'malformed')))))))
~~~
