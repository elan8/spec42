# META
~~~ini
description=General View includes filtered allocations between exposed endpoints even when the owning assembly is not exposed; contextual task occurrences remain distinct
specification=OMG SysML 2.0 (formal/2026-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=diagram_general_incident_allocation.md
viewQualifiedName=Deployment::selected
~~~
# SOURCE
~~~sysml
package Deployment {
    private import StandardViewDefinitions::*;
    part def Task;
    part def Processor;
    part def Tasks { part workerA : Task; part workerB : Task; }
    part def System {
        part tasks : Tasks;
        part cpu : Processor;
        part hidden : Processor;
        allocate tasks.workerA to cpu;
        allocate tasks.workerB to cpu;
        // An endpoint outside the view must not be invented by the projection.
        allocate tasks.workerA to hidden;
    }
    part system : System;
    view selected : GeneralView {
        filter @SysML::PartUsage or @SysML::AllocationUsage;
        expose system.tasks.workerA;
        expose system.tasks.workerB;
        expose system.cpu;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_general_incident_allocation.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:e25bbbee0535dd8ea36ce518253f7a8e9a30a9210220ff042d12850b61e5ac28") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "tasks::workerA")) (allocateTarget (reference "cpu")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "tasks::workerB")) (allocateTarget (reference "cpu")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "tasks::workerA")) (allocateTarget (reference "hidden")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Processor")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Processor")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tasks")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Task")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Task")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")) (filterMetadataTest (reference "SysML::PartUsage")) (filterMetadataTest (reference "SysML::AllocationUsage")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system::tasks::workerA")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system::tasks::workerB")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "system::cpu")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0))
      (authored-target "tasks::workerA")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateSource) (ordinal 0))
      (authored-target "tasks::workerB")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateSource) (ordinal 0))
      (authored-target "tasks::workerA")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0))
      (authored-target "cpu")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateTarget) (ordinal 0))
      (authored-target "cpu")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateTarget) (ordinal 0))
      (authored-target "hidden")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (kind featureTyping) (ordinal 0))
      (authored-target "Processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (kind featureTyping) (ordinal 0))
      (authored-target "Processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tasks")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (kind featureTyping) (ordinal 0))
      (authored-target "Task")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (kind featureTyping) (ordinal 0))
      (authored-target "Task")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind filterMetadataTest) (ordinal 1))
      (authored-target "SysML::AllocationUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "system::tasks::workerA")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0))
      (authored-target "system::tasks::workerB")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0))
      (authored-target "system::cpu")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))))
  )
  (relationships
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind filterMetadataTest) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind filterMetadataTest) (ordinal 1)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (form view) (state unsupported) (start 16 15) (end 16 59))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))
      (type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden")))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))
      (type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks")))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))
      (type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")))
      (type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB")))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")))
      (type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system")))
      (type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 9 17) (end 9 30)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0) (authored-target "tasks::workerA")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 10 17) (end 10 30)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateSource) (ordinal 0) (authored-target "tasks::workerB")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 12 17) (end 12 30)) (probe (position 12 17))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateSource) (ordinal 0) (authored-target "tasks::workerA")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 9 34) (end 9 37)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0) (authored-target "cpu")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 10 34) (end 10 37)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateTarget) (ordinal 0) (authored-target "cpu")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 12 34) (end 12 40)) (probe (position 12 34))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind part-def) (name "System")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateTarget) (ordinal 0) (authored-target "hidden")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 7 19) (end 7 28)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu"))) (kind featureTyping) (ordinal 0) (authored-target "Processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 8 22) (end 8 31)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::hidden"))) (kind featureTyping) (ordinal 0) (authored-target "Processor")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Processor")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 6 21) (end 6 26)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::tasks"))) (kind featureTyping) (ordinal 0) (authored-target "Tasks")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 4 36) (end 4 40)) (probe (position 4 36))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA"))) (kind featureTyping) (ordinal 0) (authored-target "Task")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 4 57) (end 4 61)) (probe (position 4 57))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB"))) (kind featureTyping) (ordinal 0) (authored-target "Task")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Task")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 15 20) (end 15 31)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 16 16) (end 16 32)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 16 37) (end 16 59)) (probe (position 16 37))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::selected"))) (kind filterMetadataTest) (ordinal 1) (authored-target "SysML::AllocationUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 17 15) (end 17 35)) (probe (position 17 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "system::tasks::workerA")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerA")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 18 15) (end 18 35)) (probe (position 18 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0) (authored-target "system::tasks::workerB")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::Tasks::workerB")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 19 15) (end 19 25)) (probe (position 19 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (path (named (kind package) (name "Deployment")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0) (authored-target "system::cpu")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System::cpu")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_incident_allocation.md") (range (start 14 18) (end 14 24)) (probe (position 14 18))
    (reference (id (source (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_incident_allocation.md") (qualified-name "Deployment::System")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 10,
  "modelDigest": "blake3:a024e456a22ae296b0b95709dda0d55edb7916c9b693b0155915b5179c4feb41",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_general_incident_allocation.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        4,
        26,
        4,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        36,
        4,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        47,
        4,
        54
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        57,
        4,
        61
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        13,
        7,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        19,
        7,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        8,
        9,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        17,
        9,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        34,
        9,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        8,
        10,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        17,
        10,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        34,
        10,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        9,
        15,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Deployment::Processor"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Deployment::System::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Deployment::System::cpu"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Deployment::Task"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Deployment::Tasks::workerA"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Deployment::Tasks::workerB"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Deployment::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AllocationUsage",
      "ownerQualifiedName": "Deployment::System",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AllocationUsage",
      "ownerQualifiedName": "Deployment::System",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "allocateSource",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "allocateSource",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "allocateTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "allocateTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "allocate",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "allocate",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "typing",
      "source": 5
    }
  ],
  "selectedView": {
    "reference": 6,
    "kind": "general-view",
    "name": "selected",
    "source": 12
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "allocate",
        "navigation": 7,
        "origin": 3,
        "provenance": "implied",
        "reference": 14,
        "source": 0,
        "target": 2
      },
      {
        "kind": "allocate",
        "navigation": 10,
        "origin": 4,
        "provenance": "implied",
        "reference": 16,
        "source": 1,
        "target": 2
      }
    ],
    "exposedRoots": [
      0,
      1,
      2
    ],
    "kind": "general-view",
    "metadata": {
      "roots": [
        0,
        1,
        2
      ]
    },
    "nodes": [
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "workerA",
        "notationRole": "usage",
        "owner": null,
        "reference": 4,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Task",
              "reference": 3
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "workerB",
        "notationRole": "usage",
        "owner": null,
        "reference": 5,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Task",
              "reference": 3
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "cpu",
        "notationRole": "usage",
        "owner": null,
        "reference": 2,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Processor",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AllocationUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "reference": 7,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AllocationUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "reference": 8,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "allocateSource",
        "navigation": 7,
        "provenance": "authored",
        "reference": 9,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateTarget",
        "navigation": 8,
        "provenance": "authored",
        "reference": 11,
        "source": 3,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateSource",
        "navigation": 10,
        "provenance": "authored",
        "reference": 10,
        "source": 4,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateTarget",
        "navigation": 11,
        "provenance": "authored",
        "reference": 12,
        "source": 4,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 13,
        "source": 2,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 15,
        "source": 0,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 17,
        "source": 1,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "general"
    }
  }
}

~~~
