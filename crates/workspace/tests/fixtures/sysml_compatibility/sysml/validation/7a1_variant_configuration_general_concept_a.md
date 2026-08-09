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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (name "7a1-Variant Configuration - General Concept-a") (declared-name "7a1-Variant Configuration - General Concept-a")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (name "SubsystemA") (declared-name "SubsystemA") (declared (properties (abstract true)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (name "part3") (declared (properties (abstract true) (ordered false)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (name "SubsystemB") (declared-name "SubsystemB") (declared (properties (abstract true)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (name "part5") (declared (properties (abstract true) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (name "VehicleConfigB") (declared-name "VehicleConfigB") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (name "subsystemA") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemA::subsystem2")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (name "subsystemB") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemB::subsystem4")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (role feature-value)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (name "part5") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::"))) (name ""))
                  )
                )
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (name "subsystemA") (declared-name "subsystemA") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (name "subsystem1") (declared-name "subsystem1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (name "part1") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (name "part2") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (name "subsystem2") (declared-name "subsystem2") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (name "part2") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (name "part3") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (name "subsystemB") (declared-name "subsystemB") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (name "subsystem3") (declared-name "subsystem3") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (name "part4") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (name "part5") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (name "subsystem4") (declared-name "subsystem4") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (name "part5") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (name "part6") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::doX"))) (name "doX") (declared-name "doX") (declared))
        (element (kind "action") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::doY"))) (name "doY") (declared-name "doY") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part1"))) (name "part1") (declared-name "part1") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part2"))) (name "part2") (declared-name "part2") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3"))) (name "part3") (declared-name "part3") (declared (properties (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3::p1"))) (name "p1") (declared-name "p1") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part4"))) (name "part4") (declared-name "part4") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (name "part5") (declared-name "part5") (declared (properties (ordered false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY"))) (name "doXorY") (declared-name "doXorY"))
            (element (kind "port") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::p2"))) (name "p2") (declared-name "p2") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part6"))) (name "part6") (declared-name "part6") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (name "vehicleConfigA") (declared-name "vehicleConfigA") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (name "subsystemA") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemA::subsystem1")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (name "subsystemB") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemB::subsystem3")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (role feature-value)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (name "part5") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::"))) (name ""))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (to (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (connection (status pending-expression) (document "d0") (source-expression "subsystemA::part3::p1") (target-expression "subsystemB::part5::p2") (container-prefix "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/7a1_variant_configuration_general_concept_a.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 12))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 8 2) (end 8 10))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 10 1) (end 10 12))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 12 2) (end 12 10))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 18 1) (end 18 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 21 2) (end 21 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 25 2) (end 25 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 32 4) (end 32 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 33 4) (end 33 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 36 4) (end 36 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 4) (end 43 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 48 4) (end 48 22))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 52 16) (end 52 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 52 16) (end 52 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 52 16) (end 52 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 62 2) (end 62 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 63 2) (end 63 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 64 3) (end 64 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 71 2) (end 71 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 72 2) (end 72 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 73 3) (end 73 61))
      )
    )
  )
)
~~~
