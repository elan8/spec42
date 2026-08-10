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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (name "7a-Variant Configuration - General Concept") (declared-name "7a-Variant Configuration - General Concept")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (name "VehicleConfigB") (declared-name "VehicleConfigB") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (name "subsystemA") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemA::subsystem2")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (name "subsystemB") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemB::subsystem3")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (declared (properties (abstract true) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))) (name "subsystemA") (declared-name "subsystemA") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (name "subsystem1") (declared-name "subsystem1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (name "part1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (name "part2") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (name "subsystem2") (declared-name "subsystem2") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (name "part2") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (name "part3") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))) (name "subsystemB") (declared-name "subsystemB") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (name "subsystem3") (declared-name "subsystem3") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (name "part4") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (name "part5") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (name "subsystem4") (declared-name "subsystem4") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (name "part5") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (name "part6") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part1"))) (name "part1") (declared-name "part1") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part2"))) (name "part2") (declared-name "part2") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part3"))) (name "part3") (declared-name "part3") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part4"))) (name "part4") (declared-name "part4") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part5"))) (name "part5") (declared-name "part5") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part6"))) (name "part6") (declared-name "part6") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (name "vehicleConfigA") (declared-name "vehicleConfigA") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (name "subsystemA") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemA::subsystem1")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "part") (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (name "subsystemB") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "subsystemB::subsystem3")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
          )
        )
      )
    )
  )
  (relationships
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (to (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (to (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (to (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part3"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part4"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part5"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part6"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/7a_variant_configuration_general_concept.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 4 1) (end 4 12))
      )
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
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 7 1) (end 7 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 8 1) (end 8 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 9 1) (end 9 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 15 4) (end 15 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 16 4) (end 16 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 19 4) (end 19 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 20 4) (end 20 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 26 4) (end 26 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 27 4) (end 27 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 30 4) (end 30 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 31 4) (end 31 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 2) (end 43 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 44 2) (end 44 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 48 2) (end 48 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 49 2) (end 49 47))
      )
    )
  )
)
~~~
