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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "12b-Allocation-1"))) (name "12b-Allocation-1") (declared-name "12b-Allocation-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "12b-Allocation-1::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (name "LogicalModel") (declared-name "LogicalModel")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (name "GenerateTorque") (declared-name "GenerateTorque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (name "torque") (declared-name "torque") (effective (featuring-type (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (name "LogicalElement") (declared-name "LogicalElement") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (name "TorqueGenerator") (declared-name "TorqueGenerator") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (name "providePower") (declared-name "providePower") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (name "torqueGenerator") (declared-name "torqueGenerator") (declared (properties (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
              )
            )
          )
        )
        (element (kind "allocation def") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (name "LogicalToPhysical") (declared-name "LogicalToPhysical")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (name "logical") (declared-name "logical") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (name "physical") (declared-name "physical") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical")))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))) (name "PhysicalModel") (declared-name "PhysicalModel")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (name "PhysicalElement") (declared-name "PhysicalElement") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (name "PowerTrain") (declared-name "PowerTrain") (declared))
            (element (kind "part") (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (name "powerTrain") (declared-name "powerTrain") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel"))) (name "RequirementModel") (declared-name "RequirementModel")
          (contains
            (element (kind "requirement") (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (name "torqueGeneration") (declared-name "torqueGeneration")
              (contains
                (element (kind "require constraint") (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0"))
                (element (kind "subject") (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (name "generator") (declared-name "generator"))
              )
            )
          )
        )
        (element (kind "allocation") (id (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (name "torqueGenAlloc") (declared-name "torqueGenAlloc"))
      )
    )
  )
  (relationships
    (allocate (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))))
    (satisfy (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (to (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "12b-Allocation-1::LogicalModel::torqueGenerator") (target-qualified "12b-Allocation-1::LogicalModel::torqueGenerator::providePower::generateTorque"))
    (perform (status pending) (document "d0") (source-qualified "12b-Allocation-1::PhysicalModel::powerTrain::engine") (target-qualified "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower::generateTorque"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/12b_allocation_1.md"
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
        (range (start 1 16) (end 1 18))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 16 30) (end 16 56))
      )
    )
  )
)
~~~
