# META
~~~ini
description=SysML Validation (12-Dependency Relationships): 12b-Allocation
type=file
~~~
# SOURCE
~~~sysml
package '12b-Allocation' {
	private import LogicalModel::*;
	private import PhysicalModel::*;
	
	package LogicalModel {
		action providePower {
			action generateTorque;
		}
		
		part torqueGenerator {
			perform providePower.generateTorque;
		}
	}
	
	package PhysicalModel {
		part powerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	}
	
	allocate torqueGenerator to powerTrain {
		allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwAllocate,Ident,KwTo,Ident,OpenCurly,
KwAllocate,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''12b-Allocation''
    (import_decl private 'LogicalModel::*')
    (import_decl private 'PhysicalModel::*')
    (package_def 'LogicalModel'
      (action_usage 'providePower'
        (action_usage 'generateTorque'))
      (part_usage 'torqueGenerator'
        (perform_action :>> 'providePower.generateTorque')))
    (package_def 'PhysicalModel'
      (part_usage 'powerTrain'
        (part_usage 'engine'
          (perform_action :>> 'providePower.generateTorque'))))
    (allocation_usage
      (connector_end)
      (connector_end)
      (allocation_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package '12b-Allocation' {
    private import LogicalModel::*;
    private import PhysicalModel::*;

    package LogicalModel {
        action providePower {
            action generateTorque;
        }

        part torqueGenerator {
            perform providePower.generateTorque;
        }
    }

    package PhysicalModel {
        part powerTrain {
            part engine {
                perform providePower.generateTorque;
            }
        }
    }

    allocate torqueGenerator to powerTrain {
        allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "12b-Allocation"))) (name "12b-Allocation") (declared-name "12b-Allocation")
      (contains
        (element (kind "allocation") (id (node (document "d0") (qualified-name "12b-Allocation::"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "12b-Allocation::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "12b-Allocation::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel"))) (name "LogicalModel") (declared-name "LogicalModel")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (name "providePower") (declared-name "providePower") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (declared))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (name "torqueGenerator") (declared-name "torqueGenerator") (declared (properties (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque"))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel"))) (name "PhysicalModel") (declared-name "PhysicalModel")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (name "powerTrain") (declared-name "powerTrain") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque"))
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
    (allocate (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (to (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (to (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "12b-Allocation::LogicalModel::torqueGenerator") (target-qualified "12b-Allocation::LogicalModel::torqueGenerator::providePower::generateTorque"))
    (perform (status pending) (document "d0") (source-qualified "12b-Allocation::PhysicalModel::powerTrain::engine") (target-qualified "12b-Allocation::PhysicalModel::powerTrain::engine::providePower::generateTorque"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/12b_allocation.md"
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
    )
  )
)
~~~
