# META
~~~ini
description=SysML Training 38 (Allocation): Allocation Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Allocation Definition Example' {
	package LogicalModel {
		action def ProvidePower;
		action def GenerateTorque;
		
		part def LogicalElement;
		part def TorqueGenerator :> LogicalElement;
		
		action providePower : ProvidePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque;
		}
		
	}
	
	package PhysicalModel {
		private import LogicalModel::*;
		
		part def PhysicalElement;
		part def PowerTrain :> PhysicalElement;
		
		part powerTrain : PowerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	
		allocation def LogicalToPhysical {
			end logical : LogicalElement;
			end physical : PhysicalElement;
		}
		
		allocation torqueGenAlloc : LogicalToPhysical allocate torqueGenerator to powerTrain;
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
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
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
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAllocation,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAllocation,Ident,Colon,Ident,KwAllocate,Ident,KwTo,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Allocation Definition Example''
    (package_def 'LogicalModel'
      (action_def 'ProvidePower')
      (action_def 'GenerateTorque')
      (part_def 'LogicalElement')
      (part_def 'TorqueGenerator' :> 'LogicalElement')
      (action_usage 'providePower' : 'ProvidePower'
        (action_usage 'generateTorque' : 'GenerateTorque'))
      (part_usage 'torqueGenerator' : 'TorqueGenerator'
        (perform_action :>> 'providePower.generateTorque')))
    (package_def 'PhysicalModel'
      (import_decl private 'LogicalModel::*')
      (part_def 'PhysicalElement')
      (part_def 'PowerTrain' :> 'PhysicalElement')
      (part_usage 'powerTrain' : 'PowerTrain'
        (part_usage 'engine'
          (perform_action :>> 'providePower.generateTorque')))
      (allocation_def 'LogicalToPhysical'
        (interface_end end 'logical' : 'LogicalElement')
        (interface_end end 'physical' : 'PhysicalElement'))
      (allocation_usage 'LogicalToPhysical' 'torqueGenAlloc'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Allocation Definition Example' {
    package LogicalModel {
        action def ProvidePower;
        action def GenerateTorque;

        part def LogicalElement;
        part def TorqueGenerator :> LogicalElement;

        action providePower : ProvidePower {
            action generateTorque : GenerateTorque;
        }

        part torqueGenerator : TorqueGenerator {
            perform providePower.generateTorque;
        }

    }

    package PhysicalModel {
        private import LogicalModel::*;

        part def PhysicalElement;
        part def PowerTrain :> PhysicalElement;

        part powerTrain : PowerTrain {
            part engine {
                perform providePower.generateTorque;
            }
        }

        allocation def LogicalToPhysical {
            end logical : LogicalElement;
            end physical : PhysicalElement;
        }

        allocation torqueGenAlloc : LogicalToPhysical allocate torqueGenerator to powerTrain;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Allocation Definition Example"))) (name "Allocation Definition Example") (declared-name "Allocation Definition Example")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (name "LogicalModel") (declared-name "LogicalModel")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque"))) (name "GenerateTorque") (declared-name "GenerateTorque"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))) (name "LogicalElement") (declared-name "LogicalElement") (declared))
            (element (kind "action def") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower"))) (name "ProvidePower") (declared-name "ProvidePower"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (name "TorqueGenerator") (declared-name "TorqueGenerator") (declared))
            (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (name "providePower") (declared-name "providePower") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (name "torqueGenerator") (declared-name "torqueGenerator") (declared (properties (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (name "PhysicalModel") (declared-name "PhysicalModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::*"))) (name "*") (declared-name "*"))
            (element (kind "allocation def") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (name "LogicalToPhysical") (declared-name "LogicalToPhysical")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (name "logical") (declared-name "logical") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (name "physical") (declared-name "physical") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (name "PhysicalElement") (declared-name "PhysicalElement") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (name "PowerTrain") (declared-name "PowerTrain") (declared))
            (element (kind "part") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (name "powerTrain") (declared-name "powerTrain") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")))))
                  )
                )
              )
            )
            (element (kind "allocation") (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (name "torqueGenAlloc") (declared-name "torqueGenAlloc"))
          )
        )
      )
    )
  )
  (relationships
    (allocate (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (to (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "Allocation Definition Example::LogicalModel::torqueGenerator") (target-qualified "Allocation Definition Example::LogicalModel::torqueGenerator::providePower::generateTorque"))
    (perform (status pending) (document "d0") (source-qualified "Allocation Definition Example::PhysicalModel::powerTrain::engine") (target-qualified "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower::generateTorque"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/38_allocation_definition_example.md"
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
        (range (start 19 2) (end 19 33))
      )
    )
  )
)
~~~
