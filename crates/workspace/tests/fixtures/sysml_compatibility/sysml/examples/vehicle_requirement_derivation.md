# META
~~~ini
description=SysML Example (Requirements): VehicleRequirementDerivation
type=file
~~~
# SOURCE
~~~sysml
package VehicleRequirementDerivation {
	private import RequirementDerivation::*;
	
	part vehicle {
		attribute mass :> ISQ::mass;
		
		part chassis {
			attribute mass :> ISQ::mass;
		}
		
		part engine {
			attribute mass :> ISQ::mass;
		}
	}
	
	requirement def MassRequirement {
		subject mass :> ISQ::mass;
		attribute massLimit :> ISQ::mass;
		require constraint { mass <= massLimit }
	}
	
	requirement vehicleMassRequirement : MassRequirement {
		subject :>> mass = vehicle.mass;
	}
	
	requirement chassisMassRequirement : MassRequirement {
		subject :>> mass = vehicle.chassis.mass;
	}
	
	requirement engineMassRequirement : MassRequirement {
		subject :>> mass = vehicle.engine.mass;
	}
	
	#derivation connection {
		end #original ::> vehicleMassRequirement;
		end #derive ::> chassisMassRequirement;
		end #derive ::> engineMassRequirement;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwSubject,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,LtEq,Ident,CloseCurly,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
Hash,Ident,KwConnection,OpenCurly,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,Hash,Ident,ColonColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleRequirementDerivation'
    (import_decl private 'RequirementDerivation::*')
    (part_usage 'vehicle'
      (attribute_usage 'mass' :> 'ISQ::mass')
      (part_usage 'chassis'
        (attribute_usage 'mass' :> 'ISQ::mass'))
      (part_usage 'engine'
        (attribute_usage 'mass' :> 'ISQ::mass')))
    (requirement_def 'MassRequirement'
      (sysml_decl 'mass' :> 'ISQ::mass')
      (attribute_usage 'massLimit' :> 'ISQ::mass')
      (sysml_decl
        (result_expr_member)))
    (requirement_usage 'vehicleMassRequirement' : 'MassRequirement'
      (sysml_decl :>> 'mass' value))
    (requirement_usage 'chassisMassRequirement' : 'MassRequirement'
      (sysml_decl :>> 'mass' value))
    (requirement_usage 'engineMassRequirement' : 'MassRequirement'
      (sysml_decl :>> 'mass' value))
    (malformed)
    (malformed)))
~~~
# FORMAT
~~~sysml
package VehicleRequirementDerivation {
	private import RequirementDerivation::*;
	
	part vehicle {
		attribute mass :> ISQ::mass;
		
		part chassis {
			attribute mass :> ISQ::mass;
		}
		
		part engine {
			attribute mass :> ISQ::mass;
		}
	}
	
	requirement def MassRequirement {
		subject mass :> ISQ::mass;
		attribute massLimit :> ISQ::mass;
		require constraint { mass <= massLimit }
	}
	
	requirement vehicleMassRequirement : MassRequirement {
		subject :>> mass = vehicle.mass;
	}
	
	requirement chassisMassRequirement : MassRequirement {
		subject :>> mass = vehicle.chassis.mass;
	}
	
	requirement engineMassRequirement : MassRequirement {
		subject :>> mass = vehicle.engine.mass;
	}
	
	#derivation connection {
		end #original ::> vehicleMassRequirement;
		end #derive ::> chassisMassRequirement;
		end #derive ::> engineMassRequirement;
	}
	
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (name "VehicleRequirementDerivation") (declared-name "VehicleRequirementDerivation")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::*"))) (name "*") (declared-name "*"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (name "MassRequirement") (declared-name "MassRequirement")
          (contains
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (name "massLimit") (declared-name "massLimit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
          )
        )
        (element (kind "derivation connection") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (name "_derivationConnection")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))) (name "#derive") (declared-name "#derive") (declared (properties (end true))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))) (name "#derive") (declared-name "#derive") (declared (properties (end true))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))) (name "#original") (declared-name "#original") (declared (properties (end true))))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (name "chassisMassRequirement") (declared-name "chassisMassRequirement"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (name "engineMassRequirement") (declared-name "engineMassRequirement"))
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis"))) (name "chassis") (declared-name "chassis") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement"))
      )
    )
  )
  (relationships
    (derivation (status resolved) (from (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (to (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (to (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_requirement_derivation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 41))
      )
    )
  )
)
~~~
