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
            perform :>> providePower.generateTorque;
        }
    }

    package PhysicalModel {
        private import LogicalModel::*;

        part def PowerTrain;
        part def Engine;

        part powerTrain : PowerTrain {
            part engine : Engine {
                perform :>> providePower.generateTorque;
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
(model
  (namespace
    (package 'Allocation Usage Example'
      (package 'LogicalModel'
        (action_def 'ProvidePower')
        (action_def 'GenerateTorque')
        (part_def 'TorqueGenerator')
        (action_usage 'providePower' : 'Allocation Usage Example::LogicalModel::ProvidePower'[action_def]
          (action_usage composite 'generateTorque' : 'Allocation Usage Example::LogicalModel::GenerateTorque'[action_def]))
        (part_usage 'torqueGenerator' : 'Allocation Usage Example::LogicalModel::TorqueGenerator'[part_def]
          (perform_action_usage :>> 'Allocation Usage Example::LogicalModel::providePower::generateTorque'[action_usage])))
      (package 'PhysicalModel'
        (namespace_import private -> 'Allocation Usage Example::LogicalModel'[package])
        (part_def 'PowerTrain')
        (part_def 'Engine')
        (part_usage 'powerTrain' : 'Allocation Usage Example::PhysicalModel::PowerTrain'[part_def]
          (part_usage composite 'engine' : 'Allocation Usage Example::PhysicalModel::Engine'[part_def]
            (perform_action_usage :>> 'Allocation Usage Example::LogicalModel::providePower::generateTorque'[action_usage])))
        (allocation_usage
          (connector_end 'torqueGenerator')
          (connector_end 'powerTrain')
          (allocation_usage composite
            (connector_end 'torqueGenerator.generateTorque')
            (connector_end 'powerTrain.engine.generateTorque')))))))
~~~
