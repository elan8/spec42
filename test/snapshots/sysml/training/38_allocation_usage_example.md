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
  (document "memory://snapshot/38_allocation_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 28 2) (end 30 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:203bc71c633fdc6da78269450cdcb30e3e7c97b02a20f12b4c406be0f17269b0") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ProvidePower"))))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque"))))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGenerator"))))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (path (named (kind package) (name "Allocation Usage Example")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (path (named (kind package) (name "Allocation Usage Example")) (named (kind package) (name "PhysicalModel")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LogicalModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerTrain"))))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (path (named (kind package) (name "Allocation Usage Example")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0))
      (authored-target "ProvidePower")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (path (named (kind package) (name "Allocation Usage Example")) (named (kind package) (name "PhysicalModel")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::Engine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower")))
      (supertype (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque")))
      (supertype (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator")))
      (supertype (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain")))
      (supertype (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine")))
      (supertype (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::Engine")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/38_allocation_usage_example.md") (range (start 7 24) (end 7 36)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0) (authored-target "ProvidePower")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower")))))
  )
  (query (document "memory://snapshot/38_allocation_usage_example.md") (range (start 8 27) (end 8 41)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque")))))
  )
  (query (document "memory://snapshot/38_allocation_usage_example.md") (range (start 11 25) (end 11 40)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator")))))
  )
  (query (document "memory://snapshot/38_allocation_usage_example.md") (range (start 17 17) (end 17 32)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (path (named (kind package) (name "Allocation Usage Example")) (named (kind package) (name "PhysicalModel")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::LogicalModel")))))
  )
  (query (document "memory://snapshot/38_allocation_usage_example.md") (range (start 22 20) (end 22 30)) (probe (position 22 20))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0) (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain")))))
  )
  (query (document "memory://snapshot/38_allocation_usage_example.md") (range (start 23 17) (end 23 23)) (probe (position 23 17))
    (reference (id (source (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_usage_example.md") (qualified-name "Allocation Usage Example::PhysicalModel::Engine")))))
  )
)
~~~
