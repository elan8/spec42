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
            perform :>> providePower.generateTorque;
        }
    }

    package PhysicalModel {
        part powerTrain {
            part engine {
                perform :>> providePower.generateTorque;
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
(model
  (namespace
    (package '12b-Allocation'
      (namespace_import private -> '12b-Allocation::LogicalModel'[package])
      (namespace_import private -> '12b-Allocation::PhysicalModel'[package])
      (package 'LogicalModel'
        (action_usage 'providePower'
          (action_usage composite 'generateTorque'))
        (part_usage 'torqueGenerator'
          (perform_action_usage :>> '12b-Allocation::LogicalModel::providePower::generateTorque'[action_usage])))
      (package 'PhysicalModel'
        (part_usage 'powerTrain'
          (part_usage composite 'engine'
            (perform_action_usage :>> '12b-Allocation::LogicalModel::providePower::generateTorque'[action_usage]))))
      (allocation_usage
        (connector_end 'torqueGenerator')
        (connector_end 'powerTrain')
        (allocation_usage composite
          (connector_end 'torqueGenerator.generateTorque')
          (connector_end 'powerTrain.engine.generateTorque'))))))
~~~
