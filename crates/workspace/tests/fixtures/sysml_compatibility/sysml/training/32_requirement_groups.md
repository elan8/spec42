# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Groups
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Groups' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Usages'::*;
	
	part def Engine {
		port clutchPort: ClutchPort;
		perform action generateTorque: GenerateTorque;
	}
	
	requirement vehicleSpecification {
		doc /* Overall vehicle requirements group */
		
		subject vehicle : Vehicle;
		
		require fullVehicleMassLimit;
		require emptyVehicleMassLimit;
	}
	
	requirement engineSpecification {
		doc /* Engine power requirements group */
		
		subject engine : Engine;
		
		requirement drivePowerInterface : DrivePowerInterface {
			subject = engine.clutchPort;
		}
		
		requirement torqueGeneration : TorqueGeneration {
			subject = engine.generateTorque;	
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequire,Ident,Semicolon,
KwRequire,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Requirement Groups''
    (import_decl private ''Requirement Definitions'::*')
    (import_decl private ''Requirement Usages'::*')
    (part_def 'Engine'
      (port_usage 'clutchPort' : 'ClutchPort')
      (perform_action 'generateTorque' : 'GenerateTorque'))
    (requirement_usage 'vehicleSpecification'
      (documentation)
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'fullVehicleMassLimit')
      (sysml_decl 'emptyVehicleMassLimit'))
    (requirement_usage 'engineSpecification'
      (documentation)
      (sysml_decl 'engine' : 'Engine')
      (requirement_usage 'drivePowerInterface' : 'DrivePowerInterface'
        (sysml_decl value))
      (requirement_usage 'torqueGeneration' : 'TorqueGeneration'
        (sysml_decl value)))))
~~~
# FORMAT
~~~sysml
package 'Requirement Groups' {
    private import 'Requirement Definitions'::*;
    private import 'Requirement Usages'::*;

    part def Engine {
        port clutchPort: ClutchPort;
        perform action generateTorque: GenerateTorque;
    }

    requirement vehicleSpecification {
        doc /* Overall vehicle requirements group */

        subject vehicle : Vehicle;

        require fullVehicleMassLimit;
        require emptyVehicleMassLimit;
    }

    requirement engineSpecification {
        doc /* Engine power requirements group */

        subject engine : Engine;

        requirement drivePowerInterface : DrivePowerInterface {
            subject = engine.clutchPort;
        }

        requirement torqueGeneration : TorqueGeneration {
            subject = engine.generateTorque;
        }
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'ClutchPort'
semantic.unresolved_name 'GenerateTorque'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'DrivePowerInterface'
semantic.unresolved_name 'TorqueGeneration'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ClutchPort'
semantic.unresolved_name 'GenerateTorque'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'DrivePowerInterface'
semantic.unresolved_name 'TorqueGeneration'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Requirement Groups"))) (name "Requirement Groups") (declared-name "Requirement Groups")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Groups::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Groups::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Requirement Groups::Engine::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Requirement Groups::Engine")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Groups::Engine")))))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (name "engineSpecification") (declared-name "engineSpecification")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::_documentation"))) (name ""))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (name "drivePowerInterface") (declared-name "drivePowerInterface")
              (contains
                (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (name ""))
              )
            )
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (name "engine") (declared-name "engine"))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (name "torqueGeneration") (declared-name "torqueGeneration")
              (contains
                (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (name ""))
              )
            )
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (name "vehicleSpecification") (declared-name "vehicleSpecification")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_documentation"))) (name ""))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0"))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_1"))) (name "_requireConstraint_1") (declared-name "_requireConstraint_1"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (to (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (to (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (to (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (to (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (to (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (to (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (to (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::Engine::clutchPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_0"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_1"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/32_requirement_groups.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 2) (end 23 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 2) (end 27 92))
      )
    )
  )
)
~~~
