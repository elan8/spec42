# META
~~~ini
description=Sequence reliability regression: sequence_omitted_structured; SysML 2.0 7.16.2, 8.2.3.9, 9.2.20.2.7
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=model.sysml
viewQualifiedName=Repro::selected
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Repro {
    private import StandardViewDefinitions::*;
    item def Funny;
    action def Interaction {
    part A; part B;
    message Visible from A to B;
    if true { message OmittedIf from A to B; } else { message OmittedElse from A to B; }
    while true { message OmittedWhile from A to B; }
 }
    view selected : SequenceView { expose Interaction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b7c38c25cd4ddbf37892052f3c0af5b7b51bf21c51d55c423f031320da6167b9") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Funny"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0))))) (kind while) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "A")) (flowTarget (reference "B")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "A")) (flowTarget (reference "B")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "A")) (flowTarget (reference "B")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "A")) (flowTarget (reference "B")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Interaction")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (kind flowSource) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (kind flowTarget) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (kind flowSource) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (kind flowTarget) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (kind flowSource) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (kind flowTarget) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (kind flowSource) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (kind flowTarget) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Funny"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::whileLoops"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenElseActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::whileLoopActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Funny")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::ifSubactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenElseAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenElseActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::ifSubactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::IfThenElseAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ifThenElseActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfThenElsePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::IfThenPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::loops"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::loopActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::WhileLoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::whileLoops"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::WhileLoopAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::whileLoopActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::loops")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::whileLoops")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::WhileLoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::whileLoopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::LoopPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralBooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralBooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralBooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralBooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (named (kind flow) (name "Visible")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 79) (end 6 80)) (probe (position 6 79))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (kind flowSource) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 84) (end 6 85)) (probe (position 6 84))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedElse"))))) (kind flowTarget) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 37) (end 6 38)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (kind flowSource) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 42) (end 6 43)) (probe (position 6 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind if) (ordinal 0)) (named (kind flow) (name "OmittedIf"))))) (kind flowTarget) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 43) (end 7 44)) (probe (position 7 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (kind flowSource) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 48) (end 7 49)) (probe (position 7 48))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind action-def) (name "Interaction")) (anonymous (kind while) (ordinal 0)) (named (kind flow) (name "OmittedWhile"))))) (kind flowTarget) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 25) (end 5 26)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (kind flowSource) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 30) (end 5 31)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::Visible"))) (kind flowTarget) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 20) (end 9 32)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 42) (end 9 53)) (probe (position 9 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 9,
  "modelDigest": "blake3:4f5e90bbd99fc518976d692a1c1e587fbebd0d6adb6da21dc2757814474232fa",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        15,
        3,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        9,
        4,
        10
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        17,
        4,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        12,
        5,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        25,
        5,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        30,
        5,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        4,
        6,
        88
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        22,
        6,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        37,
        6,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        42,
        6,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        62,
        6,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        79,
        6,
        80
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        84,
        6,
        85
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        4,
        7,
        52
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        25,
        7,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        43,
        7,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        48,
        7,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        9,
        9,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::::OmittedElse"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::::OmittedElse::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::::OmittedIf"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::::OmittedIf::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::::OmittedWhile"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::::OmittedWhile::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::A"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::B"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::Visible"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::Visible::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "IfActionUsage",
      "ownerQualifiedName": "Repro::Interaction",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "WhileLoopActionUsage",
      "ownerQualifiedName": "Repro::Interaction",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::::OmittedElse",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::::OmittedElse",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::::OmittedElse::",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::::OmittedElse::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::::OmittedIf",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::::OmittedIf",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::::OmittedIf::",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::::OmittedIf::",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::::OmittedWhile",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::::OmittedWhile",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::::OmittedWhile::",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::::OmittedWhile::",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::Visible",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::Visible",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::Visible::",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::Visible::",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "flowSource",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "flowTarget",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "flowSource",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "flowTarget",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "flowSource",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "flowTarget",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "flow",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "flow",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "flow",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "flow",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "flowSource",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "flowTarget",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "containment",
      "source": 11
    }
  ],
  "selectedView": {
    "reference": 12,
    "kind": "sequence-view",
    "name": "selected",
    "source": 17
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "sequence-content-omitted",
        "elements": [
          2,
          7,
          20
        ]
      }
    ]
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 1,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 7,
        "provenance": "authored",
        "reference": 36,
        "source": 1,
        "target": 7
      },
      {
        "kind": "flow",
        "navigation": 8,
        "origin": 7,
        "provenance": "implied",
        "reference": 57,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 8,
        "provenance": "authored",
        "reference": 45,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 9,
        "provenance": "authored",
        "reference": 49,
        "source": 8,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 10,
        "provenance": "authored",
        "reference": 46,
        "source": 7,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 11,
        "provenance": "authored",
        "reference": 50,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 2,
        "provenance": "authored",
        "reference": 37,
        "source": 1,
        "target": 2
      },
      {
        "kind": "flow",
        "navigation": 11,
        "origin": 2,
        "provenance": "implied",
        "reference": 58,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 3,
        "provenance": "authored",
        "reference": 39,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 4,
        "provenance": "authored",
        "reference": 43,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 5,
        "provenance": "authored",
        "reference": 40,
        "source": 2,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 6,
        "provenance": "authored",
        "reference": 44,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 19,
        "provenance": "authored",
        "reference": 32,
        "source": 0,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 20,
        "provenance": "authored",
        "reference": 38,
        "source": 19,
        "target": 20
      },
      {
        "kind": "flow",
        "navigation": 15,
        "origin": 20,
        "provenance": "implied",
        "reference": 59,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 21,
        "provenance": "authored",
        "reference": 51,
        "source": 20,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 22,
        "provenance": "authored",
        "reference": 55,
        "source": 21,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 23,
        "provenance": "authored",
        "reference": 52,
        "source": 20,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 24,
        "provenance": "authored",
        "reference": 56,
        "source": 23,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 17,
        "provenance": "authored",
        "reference": 33,
        "source": 0,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 18,
        "provenance": "authored",
        "reference": 34,
        "source": 0,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 12,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": 12
      },
      {
        "kind": "flow",
        "navigation": 4,
        "origin": 12,
        "provenance": "implied",
        "reference": 60,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 13,
        "provenance": "authored",
        "reference": 61,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 14,
        "provenance": "authored",
        "reference": 65,
        "source": 13,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 15,
        "provenance": "authored",
        "reference": 62,
        "source": 12,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 16,
        "provenance": "authored",
        "reference": 66,
        "source": 15,
        "target": 16
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        12
      ],
      "participants": [
        17,
        18
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              17,
              18
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              1,
              19
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              12
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionDefinition",
        "multiplicity": null,
        "name": "Interaction",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "connections",
            "members": [
              2,
              7
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "IfActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 13,
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
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": "OmittedElse",
        "notationRole": "usage",
        "owner": 1,
        "reference": 2,
        "source": 10,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "reference": 15,
        "source": 11,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 3,
        "reference": 17,
        "source": 11,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "reference": 16,
        "source": 12,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 5,
        "reference": 18,
        "source": 12,
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
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": "OmittedIf",
        "notationRole": "usage",
        "owner": 1,
        "reference": 4,
        "source": 7,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 7,
        "reference": 19,
        "source": 8,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 8,
        "reference": 21,
        "source": 8,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 7,
        "reference": 20,
        "source": 9,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 10,
        "reference": 22,
        "source": 9,
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
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": "Visible",
        "notationRole": "usage",
        "owner": 0,
        "reference": 10,
        "source": 3,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 12,
        "reference": 27,
        "source": 4,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 13,
        "reference": 29,
        "source": 4,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 12,
        "reference": 28,
        "source": 5,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 15,
        "reference": 30,
        "source": 5,
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
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "A",
        "notationRole": "usage",
        "owner": 0,
        "reference": 8,
        "source": 1,
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
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "B",
        "notationRole": "usage",
        "owner": 0,
        "reference": 9,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "connections",
            "members": [
              20
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "WhileLoopActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 14,
        "source": 13,
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
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": "OmittedWhile",
        "notationRole": "usage",
        "owner": 19,
        "reference": 6,
        "source": 14,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 20,
        "reference": 23,
        "source": 15,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 21,
        "reference": 25,
        "source": 15,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 20,
        "reference": 24,
        "source": 16,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 23,
        "reference": 26,
        "source": 16,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "flowSource",
        "navigation": 8,
        "provenance": "authored",
        "reference": 47,
        "source": 7,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 9,
        "provenance": "authored",
        "reference": 48,
        "source": 7,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 11,
        "provenance": "authored",
        "reference": 41,
        "source": 2,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 12,
        "provenance": "authored",
        "reference": 42,
        "source": 2,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 15,
        "provenance": "authored",
        "reference": 53,
        "source": 20,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 16,
        "provenance": "authored",
        "reference": 54,
        "source": 20,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 4,
        "provenance": "authored",
        "reference": 63,
        "source": 12,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 5,
        "provenance": "authored",
        "reference": 64,
        "source": 12,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        17,
        18
      ],
      "messages": [
        {
          "label": "Visible",
          "navigation": 3,
          "node": 12,
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "source": {
            "lifeline": 17,
            "status": "resolved"
          },
          "target": {
            "lifeline": 18,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
