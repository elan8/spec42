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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12b_allocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 2) (end 9 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 3) (end 16 62))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "130ae23fece4382a7835e2250e6e680c6db6acae6a949ffdeb72b3d2d9acf861") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "12b-Allocation"))) (kind "package") (name "12b-Allocation") (declared-name "12b-Allocation") (range (start (line 0) (character 0)) (end (line 0) (character 493))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::"))) (kind "allocation") (name "") (range (start (line 22) (character 1)) (end (line 22) (character 123))) (parent (node (document "d0") (qualified-name "12b-Allocation"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "12b-Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 33))) (parent (node (document "d0") (qualified-name "12b-Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "PhysicalModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 29))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (range (start (line 4) (character 1)) (end (line 4) (character 152))) (parent (node (document "d0") (qualified-name "12b-Allocation"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (range (start (line 5) (character 2)) (end (line 5) (character 53))) (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation::LogicalModel::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (range (start (line 6) (character 3)) (end (line 6) (character 25))) (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (range (start (line 9) (character 2)) (end (line 9) (character 68))) (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation::LogicalModel::torqueGenerator::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 10) (character 3)) (end (line 10) (character 39))) (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (range (start (line 14) (character 1)) (end (line 14) (character 114))) (parent (node (document "d0") (qualified-name "12b-Allocation"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (range (start (line 15) (character 2)) (end (line 15) (character 86))) (parent (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 16) (character 3)) (end (line 16) (character 62))) (parent (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 17) (character 4)) (end (line 17) (character 40))) (parent (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (range (start (line 22) (character 10)) (end (line 22) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (range (start (line 22) (character 29)) (end (line 22) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "PhysicalModel::*") (range (start (line 2) (character 16)) (end (line 2) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation::LogicalModel::providePower::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation::LogicalModel::torqueGenerator::providePower::generateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind performSource) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "torqueGenerator") (target "powerTrain") (source-range (start (line 22) (character 10)) (end (line 22) (character 25))) (target-range (start (line 22) (character 29)) (end (line 22) (character 39)))))
  )
  (evaluation
  )
)
~~~
