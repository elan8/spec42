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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/38_allocation_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 11) (end 13 38))
      )
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 19 17) (end 19 32))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 25 3) (end 27 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 12) (end 26 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f787c0ba6010bc10e009eb5b197e0df19951e5645283ebc65e671aa97e4292ec") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LogicalElement")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ProvidePower")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGenerator")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "providePower::generateTorque")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LogicalModel") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (kind allocation-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LogicalElement")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PhysicalElement")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PhysicalElement")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerTrain")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "providePower::generateTorque")))))
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LogicalToPhysical")) (allocateSource (reference "torqueGenerator")) (allocateTarget (reference "powerTrain")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0))
      (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0))
      (authored-target "ProvidePower")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "providePower::generateTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0))
      (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0))
      (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0))
      (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "providePower::generateTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind featureTyping) (ordinal 0))
      (authored-target "LogicalToPhysical")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind allocateSource) (ordinal 0))
      (authored-target "torqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind allocateTarget) (ordinal 0))
      (authored-target "powerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower")))
      (type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")) (provenance authored))
      (effective-type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")) (source direct))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque")))
      (featured-by (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower")))
      (type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")) (provenance authored))
      (effective-type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")) (source direct))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator")))
      (type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")) (provenance authored))
      (effective-type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")) (source direct))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")) (scopes any))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator")))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical")))
      (featured-by (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))
      (type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")) (provenance authored))
      (effective-type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")) (source direct))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical")))
      (featured-by (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))
      (type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")) (provenance authored))
      (effective-type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")) (source direct))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical")) (scopes any))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain")))
      (type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")) (provenance authored))
      (effective-type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")) (source direct))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")) (scopes any))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine")))
      (featured-by (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain")))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine")))
    )
    (declaration (id (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc")))
      (positional-ends (authored 0) (effective 2))
      (type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")) (provenance authored))
      (effective-type (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")) (source direct))
      (supertype (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 6 30) (end 6 44)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0) (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 8 24) (end 8 36)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0) (authored-target "ProvidePower")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 9 27) (end 9 41)) (probe (position 9 27))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 12 25) (end 12 40)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 13 11) (end 13 38)) (probe (position 13 11))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "providePower::generateTorque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 19 17) (end 19 32)) (probe (position 19 17))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 31 17) (end 31 31)) (probe (position 31 17))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0) (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 32 18) (end 32 33)) (probe (position 32 18))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0) (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 22 25) (end 22 40)) (probe (position 22 25))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0) (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 24 20) (end 24 30)) (probe (position 24 20))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0) (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 26 12) (end 26 39)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (path (named (kind package) (name "Allocation Definition Example")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "providePower::generateTorque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 35 30) (end 35 47)) (probe (position 35 30))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind featureTyping) (ordinal 0) (authored-target "LogicalToPhysical")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 35 57) (end 35 72)) (probe (position 35 57))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind allocateSource) (ordinal 0) (authored-target "torqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator")))))
    )
  )
  (query (document "memory://snapshot/38_allocation_definition_example.md") (range (start 35 76) (end 35 86)) (probe (position 35 76))
    (reference (id (source (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind allocateTarget) (ordinal 0) (authored-target "powerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/38_allocation_definition_example.md") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain")))))
    )
  )
)
~~~
