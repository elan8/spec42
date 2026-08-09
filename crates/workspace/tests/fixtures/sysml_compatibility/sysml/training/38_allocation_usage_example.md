# META
~~~ini
description=SysML Training 38 (Allocation): Allocation Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Allocation Usage Example' {
	package LogicalModel {
		action def ProvidePower;
		action def GenerateTorque;
		
		part def TorqueGenerator;
		
		action providePower : ProvidePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque;
		}
	}
	
	package PhysicalModel {
		private import LogicalModel::*;
	
		part def PowerTrain;
		part def Engine;
		
		part powerTrain : PowerTrain {
			part engine : Engine {
				perform providePower.generateTorque;
			}
		}
		
		allocate torqueGenerator to powerTrain {
			allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAllocate,Ident,KwTo,Ident,OpenCurly,
KwAllocate,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Allocation Usage Example''
    (package_def 'LogicalModel'
      (action_def 'ProvidePower')
      (action_def 'GenerateTorque')
      (part_def 'TorqueGenerator')
      (action_usage 'providePower' : 'ProvidePower'
        (action_usage 'generateTorque' : 'GenerateTorque'))
      (part_usage 'torqueGenerator' : 'TorqueGenerator'
        (perform_action :>> 'providePower.generateTorque')))
    (package_def 'PhysicalModel'
      (import_decl private 'LogicalModel::*')
      (part_def 'PowerTrain')
      (part_def 'Engine')
      (part_usage 'powerTrain' : 'PowerTrain'
        (part_usage 'engine' : 'Engine'
          (perform_action :>> 'providePower.generateTorque')))
      (allocation_usage
        (connector_end)
        (connector_end)
        (allocation_usage
          (connector_end)
          (connector_end))))))
~~~
# FORMAT
~~~sysml
package 'Allocation Usage Example' {
    package LogicalModel {
        action def ProvidePower;
        action def GenerateTorque;

        part def TorqueGenerator;

        action providePower : ProvidePower {
            action generateTorque : GenerateTorque;
        }

        part torqueGenerator : TorqueGenerator {
            perform providePower.generateTorque;
        }
    }

    package PhysicalModel {
        private import LogicalModel::*;

        part def PowerTrain;
        part def Engine;

        part powerTrain : PowerTrain {
            part engine : Engine {
                perform providePower.generateTorque;
            }
        }

        allocate torqueGenerator to powerTrain {
            allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
        }
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Allocation Usage Example"))) (name "Allocation Usage Example") (declared-name "Allocation Usage Example")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))) (name "LogicalModel") (declared-name "LogicalModel")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (name "GenerateTorque") (declared-name "GenerateTorque"))
            (element (kind "action def") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (name "ProvidePower") (declared-name "ProvidePower"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (name "TorqueGenerator") (declared-name "TorqueGenerator") (declared))
            (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (name "providePower") (declared-name "providePower") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (name "torqueGenerator") (declared-name "torqueGenerator") (declared (properties (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (name "PhysicalModel") (declared-name "PhysicalModel")
          (contains
            (element (kind "allocation") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::"))) (name ""))
            (element (kind "import") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::*"))) (name "*") (declared-name "*"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (name "Engine") (declared-name "Engine") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (name "PowerTrain") (declared-name "PowerTrain") (declared))
            (element (kind "part") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (name "powerTrain") (declared-name "powerTrain") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine")))))
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
    (allocate (status resolved) (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (to (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (to (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (to (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (to (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (to (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (to (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (to (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (provenance authored))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "Allocation Usage Example::LogicalModel::torqueGenerator") (target-qualified "Allocation Usage Example::LogicalModel::torqueGenerator::providePower::generateTorque"))
    (perform (status pending) (document "d0") (source-qualified "Allocation Usage Example::PhysicalModel::powerTrain::engine") (target-qualified "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower::generateTorque"))
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator::providePower.generateTorque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::"))) (status missing-prerequisite) (target "Allocations::allocations"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/38_allocation_usage_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 17) (end 17 29))
      )
    )
  )
)
~~~
