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
            perform :>> providePower.generateTorque;
        }
    }

    package PhysicalModel {
        private import LogicalModel::*;

        part def PhysicalElement;
        part def PowerTrain :> PhysicalElement;

        part powerTrain : PowerTrain {
            part engine {
                perform :>> providePower.generateTorque;
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
(model
  (namespace
    (package 'Allocation Definition Example'
      (package 'LogicalModel'
        (action_def 'ProvidePower')
        (action_def 'GenerateTorque')
        (part_def 'LogicalElement')
        (part_def 'TorqueGenerator' :> 'Allocation Definition Example::LogicalModel::LogicalElement'[part_def])
        (action_usage 'providePower' : 'Allocation Definition Example::LogicalModel::ProvidePower'[action_def]
          (action_usage composite 'generateTorque' : 'Allocation Definition Example::LogicalModel::GenerateTorque'[action_def]))
        (part_usage 'torqueGenerator' : 'Allocation Definition Example::LogicalModel::TorqueGenerator'[part_def]
          (perform_action_usage :>> 'Allocation Definition Example::LogicalModel::providePower::generateTorque'[action_usage])))
      (package 'PhysicalModel'
        (namespace_import private -> 'Allocation Definition Example::LogicalModel'[package])
        (part_def 'PhysicalElement')
        (part_def 'PowerTrain' :> 'Allocation Definition Example::PhysicalModel::PhysicalElement'[part_def])
        (part_usage 'powerTrain' : 'Allocation Definition Example::PhysicalModel::PowerTrain'[part_def]
          (part_usage composite 'engine'
            (perform_action_usage :>> 'Allocation Definition Example::LogicalModel::providePower::generateTorque'[action_usage])))
        (allocation_def 'LogicalToPhysical'
          (port_usage end 'logical' : 'Allocation Definition Example::LogicalModel::LogicalElement'[part_def])
          (port_usage end 'physical' : 'Allocation Definition Example::PhysicalModel::PhysicalElement'[part_def]))
        (allocation_usage 'torqueGenAlloc' : 'Allocation Definition Example::PhysicalModel::LogicalToPhysical'[allocation_def]
          (connector_end 'torqueGenerator')
          (connector_end 'powerTrain'))))))
~~~
