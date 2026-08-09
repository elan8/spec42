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
        require constraint {
            = mass <= massLimit;
        }
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

    #derivation
    connection {
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
(model
  (namespace
    (package 'VehicleRequirementDerivation'
      (namespace_import private -> 'RequirementDerivation'[unresolved])
      (part_usage 'vehicle'
        (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])
        (part_usage composite 'chassis'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
        (part_usage composite 'engine'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])))
      (requirement_def 'MassRequirement'
        (subject_membership in 'mass' :> 'ISQ::mass'[unresolved])
        (attribute_usage composite 'massLimit' :> 'ISQ::mass'[unresolved])
        (require_constraint_usage composite
          (result_expr_membership)))
      (requirement_usage 'vehicleMassRequirement' : 'VehicleRequirementDerivation::MassRequirement'[requirement_def]
        (subject_membership in :>> 'VehicleRequirementDerivation::MassRequirement::mass'[subject_membership]
          (feature_value (=))))
      (requirement_usage 'chassisMassRequirement' : 'VehicleRequirementDerivation::MassRequirement'[requirement_def]
        (subject_membership in :>> 'VehicleRequirementDerivation::MassRequirement::mass'[subject_membership]
          (feature_value (=))))
      (requirement_usage 'engineMassRequirement' : 'VehicleRequirementDerivation::MassRequirement'[requirement_def]
        (subject_membership in :>> 'VehicleRequirementDerivation::MassRequirement::mass'[subject_membership]
          (feature_value (=))))
      (not_implemented 'malformed')
      (not_implemented 'malformed'))))
~~~
