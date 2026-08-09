# META
~~~ini
description=SysML Validation (12-Dependency Relationships): 12b-Allocation-1
type=file
~~~
# SOURCE
~~~sysml
package '12b-Allocation-1' {
	private import SI::*;
	private import RequirementModel::*;
	private import LogicalModel::*;
	private import PhysicalModel::*;
	
	package RequirementModel {
		requirement torqueGeneration {
			subject generator: TorqueGenerator;
			require constraint { 
				 generator.generateTorque.torque > 0.0 [N*m]
			}
		}
	}
	
	package LogicalModel {
		action def GenerateTorque { out torque :> ISQ::torque; }
		
		part def LogicalElement;
		part def TorqueGenerator :> LogicalElement {
			perform action generateTorque : GenerateTorque;
		}	
		
		action providePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque :>> generateTorque;
		}
		
		satisfy torqueGeneration by torqueGenerator;			
	}
	
	package PhysicalModel {
		part def PhysicalElement;
		part def PowerTrain :> PhysicalElement;
		
		part powerTrain : PowerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	}
	
	allocation def LogicalToPhysical {
		end logical : LogicalElement;
		end physical : PhysicalElement;
	}
	
	allocation torqueGenAlloc : LogicalToPhysical 
		allocate logical ::> torqueGenerator to physical ::> powerTrain {
			
		allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwRequirement,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,
Ident,Dot,Ident,Dot,Ident,CloseAngle,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Star,Ident,CloseSquare,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,OpenCurly,KwOut,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwAllocation,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAllocation,Ident,Colon,Ident,
KwAllocate,Ident,ColonColonGt,Ident,KwTo,Ident,ColonColonGt,Ident,OpenCurly,
KwAllocate,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''12b-Allocation-1''
    (import_decl private 'SI::*')
    (import_decl private 'RequirementModel::*')
    (import_decl private 'LogicalModel::*')
    (import_decl private 'PhysicalModel::*')
    (package_def 'RequirementModel'
      (requirement_usage 'torqueGeneration'
        (sysml_decl 'generator' : 'TorqueGenerator')
        (sysml_decl
          (result_expr_member))))
    (package_def 'LogicalModel'
      (action_def 'GenerateTorque'
        (default_ref_usage out 'torque' :> 'ISQ::torque'))
      (part_def 'LogicalElement')
      (part_def 'TorqueGenerator' :> 'LogicalElement'
        (perform_action 'generateTorque' : 'GenerateTorque'))
      (action_usage 'providePower'
        (action_usage 'generateTorque' : 'GenerateTorque'))
      (part_usage 'torqueGenerator' : 'TorqueGenerator'
        (perform_action :>> 'providePower.generateTorque')
        (default_ref_usage :>> 'generateTorque'))
      (sysml_decl 'torqueGeneration'))
    (package_def 'PhysicalModel'
      (part_def 'PhysicalElement')
      (part_def 'PowerTrain' :> 'PhysicalElement')
      (part_usage 'powerTrain' : 'PowerTrain'
        (part_usage 'engine'
          (perform_action :>> 'providePower.generateTorque'))))
    (allocation_def 'LogicalToPhysical'
      (interface_end end 'logical' : 'LogicalElement')
      (interface_end end 'physical' : 'PhysicalElement'))
    (allocation_usage 'LogicalToPhysical' 'torqueGenAlloc'
      (connector_end)
      (connector_end)
      (allocation_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package '12b-Allocation-1' {
    private import SI::*;
    private import RequirementModel::*;
    private import LogicalModel::*;
    private import PhysicalModel::*;

    package RequirementModel {
        requirement torqueGeneration {
            subject generator : TorqueGenerator;
            require constraint {
                = generator.generateTorque.torque > 0.0[N * m];
            }
        }
    }

    package LogicalModel {
        action def GenerateTorque {
            out torque :> ISQ::torque;
        }

        part def LogicalElement;
        part def TorqueGenerator :> LogicalElement {
            perform action generateTorque : GenerateTorque;
        }

        action providePower {
            action generateTorque : GenerateTorque;
        }

        part torqueGenerator : TorqueGenerator {
            perform :>> providePower.generateTorque;
            :>> generateTorque;
        }

        satisfy torqueGeneration by torqueGenerator;
    }

    package PhysicalModel {
        part def PhysicalElement;
        part def PowerTrain :> PhysicalElement;

        part powerTrain : PowerTrain {
            part engine {
                perform :>> providePower.generateTorque;
            }
        }
    }

    allocation def LogicalToPhysical {
        end logical : LogicalElement;
        end physical : PhysicalElement;
    }

    allocation torqueGenAlloc : LogicalToPhysical allocate logical ::> torqueGenerator to physical ::> powerTrain {
        allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::torque'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::torque'
~~~
# SMG
~~~
(model
  (namespace
    (package '12b-Allocation-1'
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> '12b-Allocation-1::RequirementModel'[package])
      (namespace_import private -> '12b-Allocation-1::LogicalModel'[package])
      (namespace_import private -> '12b-Allocation-1::PhysicalModel'[package])
      (package 'RequirementModel'
        (requirement_usage 'torqueGeneration'
          (subject_membership in 'generator' : '12b-Allocation-1::LogicalModel::TorqueGenerator'[part_def])
          (require_constraint_usage composite
            (result_expr_membership))))
      (package 'LogicalModel'
        (action_def 'GenerateTorque'
          (reference_usage out reference 'torque' :> 'ISQ::torque'[unresolved]))
        (part_def 'LogicalElement')
        (part_def 'TorqueGenerator' :> '12b-Allocation-1::LogicalModel::LogicalElement'[part_def]
          (perform_action_usage 'generateTorque' : '12b-Allocation-1::LogicalModel::GenerateTorque'[action_def]))
        (action_usage 'providePower'
          (action_usage composite 'generateTorque' : '12b-Allocation-1::LogicalModel::GenerateTorque'[action_def]))
        (part_usage 'torqueGenerator' : '12b-Allocation-1::LogicalModel::TorqueGenerator'[part_def]
          (perform_action_usage :>> '12b-Allocation-1::LogicalModel::providePower::generateTorque'[action_usage])
          (reference_usage reference :>> '12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque'[perform_action_usage]))
        (satisfy_requirement_usage 'torqueGeneration' by '12b-Allocation-1::LogicalModel::torqueGenerator'[part_usage]))
      (package 'PhysicalModel'
        (part_def 'PhysicalElement')
        (part_def 'PowerTrain' :> '12b-Allocation-1::PhysicalModel::PhysicalElement'[part_def])
        (part_usage 'powerTrain' : '12b-Allocation-1::PhysicalModel::PowerTrain'[part_def]
          (part_usage composite 'engine'
            (perform_action_usage :>> '12b-Allocation-1::LogicalModel::providePower::generateTorque'[action_usage]))))
      (allocation_def 'LogicalToPhysical'
        (port_usage end 'logical' : '12b-Allocation-1::LogicalModel::LogicalElement'[part_def])
        (port_usage end 'physical' : '12b-Allocation-1::PhysicalModel::PhysicalElement'[part_def]))
      (allocation_usage 'torqueGenAlloc' : '12b-Allocation-1::LogicalToPhysical'[allocation_def]
        (connector_end 'logical' :> '12b-Allocation-1::LogicalModel::torqueGenerator'[part_usage])
        (connector_end 'physical' :> '12b-Allocation-1::PhysicalModel::powerTrain'[part_usage])
        (allocation_usage composite
          (connector_end 'torqueGenerator.generateTorque')
          (connector_end 'powerTrain.engine.generateTorque'))))))
~~~
