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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "38_allocation_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 2) (end 11 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 17) (end 17 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 3) (end 23 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 11) (end 28 26))
      )
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4ea1c9cd5e3312e99a80a01351837ff03dff6b5ecbc9fc650064aa276c759226") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example"))) (kind "package") (name "Allocation Usage Example") (declared-name "Allocation Usage Example") (range (start (line 0) (character 0)) (end (line 0) (character 683))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (range (start (line 1) (character 1)) (end (line 1) (character 292))) (parent (node (document "d0") (qualified-name "Allocation Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (range (start (line 3) (character 2)) (end (line 3) (character 28))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (kind "action def") (name "ProvidePower") (declared-name "ProvidePower") (range (start (line 2) (character 2)) (end (line 2) (character 26))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (kind "part def") (name "TorqueGenerator") (declared-name "TorqueGenerator") (range (start (line 5) (character 2)) (end (line 5) (character 27))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (range (start (line 7) (character 2)) (end (line 7) (character 85))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "ProvidePower") (range none)) (perform (reference "Allocation Usage Example::LogicalModel::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (range (start (line 8) (character 3)) (end (line 8) (character 42))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (range (start (line 11) (character 2)) (end (line 11) (character 86))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGenerator") (range (start (line 11) (character 25)) (end (line 11) (character 40)))) (perform (reference "Allocation Usage Example::LogicalModel::torqueGenerator::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 12) (character 3)) (end (line 12) (character 39))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (range (start (line 16) (character 1)) (end (line 16) (character 347))) (parent (node (document "d0") (qualified-name "Allocation Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::"))) (kind "allocation") (name "") (range (start (line 28) (character 2)) (end (line 28) (character 126))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 2)) (end (line 17) (character 33))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 17)) (end (line 17) (character 29))))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 20) (character 2)) (end (line 20) (character 18))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (kind "part def") (name "PowerTrain") (declared-name "PowerTrain") (range (start (line 19) (character 2)) (end (line 19) (character 22))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (range (start (line 22) (character 2)) (end (line 22) (character 108))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerTrain") (range (start (line 22) (character 20)) (end (line 22) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 23) (character 3)) (end (line 23) (character 71))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 23) (character 17)) (end (line 23) (character 23)))) (perform (reference "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 24) (character 4)) (end (line 24) (character 40))) (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)) (authored-target "ProvidePower") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Usage Example::LogicalModel::providePower::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (range (start (line 11) (character 25)) (end (line 11) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Usage Example::LogicalModel::torqueGenerator::providePower::generateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (range (start (line 28) (character 11)) (end (line 28) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (range (start (line 28) (character 30)) (end (line 28) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (range (start (line 17) (character 17)) (end (line 17) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerTrain") (range (start (line 22) (character 20)) (end (line 22) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 23) (character 17)) (end (line 23) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
