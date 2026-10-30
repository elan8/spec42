# META
~~~ini
description=Sequence view exposes cyclic message ordering as an incomplete typed scene
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_ordering_cycle.md
viewQualifiedName=SequenceCycle::selected
~~~
# SOURCE
~~~sysml
package SequenceCycle {
    private import StandardViewDefinitions::*;
    occurrence def Interaction {
        part left { event occurrence 'send'; event occurrence receive; }
        part right { event occurrence 'send'; event occurrence receive; }
        message 'first' from left.'send' to right.receive;
        message second from right.'send' to left.receive;
        succession first 'first' then second;
        succession first second then 'first';
    }
    view selected : SequenceView { expose Interaction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_ordering_cycle.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8f391ea942b8a4f525e535f9842cc47bbebd50c71e6af111bb98dd2ea0c3239e") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "first")) (succession (reference "second")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "second")) (succession (reference "first")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "left::send")) (flowTarget (reference "right::receive")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "right::send")) (flowTarget (reference "left::receive")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Interaction")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowSource) (ordinal 0))
      (authored-target "left::send")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowTarget) (ordinal 0))
      (authored-target "right::receive")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flowSource) (ordinal 0))
      (authored-target "right::send")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flowTarget) (ordinal 0))
      (authored-target "left::receive")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "first")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "second")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 7 25) (end 7 32)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 8 25) (end 8 31)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 7 38) (end 7 44)) (probe (position 7 38))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 8 37) (end 8 44)) (probe (position 8 37))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 5 29) (end 5 40)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowSource) (ordinal 0) (authored-target "left::send")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 5 44) (end 5 57)) (probe (position 5 44))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowTarget) (ordinal 0) (authored-target "right::receive")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 6 28) (end 6 40)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flowSource) (ordinal 0) (authored-target "right::send")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 6 44) (end 6 56)) (probe (position 6 44))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flowTarget) (ordinal 0) (authored-target "left::receive")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 10 20) (end 10 32)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 10 42) (end 10 53)) (probe (position 10 42))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:d4092718811d0038c2c90aa2b6903cf047a1f4c95bd27b4bb1f13be95555ec6a",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_ordering_cycle.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        2,
        19,
        2,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        38,
        3,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        62,
        3,
        69
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        39,
        4,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        63,
        4,
        70
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        17,
        5,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        29,
        5,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        44,
        5,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        16,
        6,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        28,
        6,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        44,
        6,
        56
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        25,
        7,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        38,
        7,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        8,
        8,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        25,
        8,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        37,
        8,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        9,
        10,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::first"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::first::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::left"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::left::receive"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::left::send"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::right"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::right::receive"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::right::send"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::second"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::Interaction::second::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "SequenceCycle::Interaction",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "SequenceCycle::Interaction",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceCycle::Interaction::first",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceCycle::Interaction::first",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceCycle::Interaction::first::",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceCycle::Interaction::first::",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceCycle::Interaction::second",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceCycle::Interaction::second",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceCycle::Interaction::second::",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceCycle::Interaction::second::",
      "source": 12,
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
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 7,
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
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "succession",
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
      "ordinal": 16,
      "relationshipKind": "flowSource",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "flowTarget",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "succession",
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
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "flow",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "flow",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "flowSource",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "flowTarget",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 11
    }
  ],
  "selectedView": {
    "reference": 12,
    "kind": "sequence-view",
    "name": "selected",
    "source": 19
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "sequence-ordering-cycle",
        "witness": [
          {
            "events": [],
            "provenance": "authored",
            "source": 3,
            "target": 8
          },
          {
            "events": [],
            "provenance": "authored",
            "source": 8,
            "target": 3
          }
        ]
      }
    ]
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 13,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 14,
        "provenance": "authored",
        "reference": 40,
        "source": 13,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 15,
        "provenance": "authored",
        "reference": 41,
        "source": 13,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 1,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 1,
        "provenance": "implied",
        "reference": 37,
        "source": 3,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 2,
        "provenance": "authored",
        "reference": 25,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 17,
        "origin": 2,
        "provenance": "implied",
        "reference": 50,
        "source": 8,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 3,
        "provenance": "authored",
        "reference": 26,
        "source": 0,
        "target": 3
      },
      {
        "kind": "flow",
        "navigation": 8,
        "origin": 3,
        "provenance": "implied",
        "reference": 42,
        "source": 14,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 4,
        "provenance": "authored",
        "reference": 33,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 5,
        "provenance": "authored",
        "reference": 38,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 6,
        "provenance": "authored",
        "reference": 34,
        "source": 3,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 7,
        "provenance": "authored",
        "reference": 39,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 16,
        "provenance": "authored",
        "reference": 27,
        "source": 0,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 17,
        "provenance": "authored",
        "reference": 43,
        "source": 16,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 18,
        "provenance": "authored",
        "reference": 44,
        "source": 16,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 8,
        "provenance": "authored",
        "reference": 28,
        "source": 0,
        "target": 8
      },
      {
        "kind": "flow",
        "navigation": 11,
        "origin": 8,
        "provenance": "implied",
        "reference": 45,
        "source": 17,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 9,
        "provenance": "authored",
        "reference": 46,
        "source": 8,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 10,
        "provenance": "authored",
        "reference": 51,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 11,
        "provenance": "authored",
        "reference": 47,
        "source": 8,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 12,
        "provenance": "authored",
        "reference": 52,
        "source": 11,
        "target": 12
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        3,
        8
      ],
      "participants": [
        13,
        16
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              13,
              16
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              3,
              8
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "OccurrenceDefinition",
        "multiplicity": null,
        "name": "Interaction",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:5f801196751832be81fac4bbd72aef0d92701dd5b86a5d9088bdfc9c689c43ec",
        "reference": 0,
        "source": 0,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:c24adecef1058db7b1d5d67c4fb1a38e0348da9617a927596efa7152180bed65",
        "reference": 13,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:4a7c208fcf3e5c876aa4bc50c79529cf4e7d438c90efd08974b5c43ed79a7919",
        "reference": 14,
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
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": "first",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:ba2ef9ccfc96a77151744fc676a38fe9735023a54c401a2b2b3fc8659537a201",
        "reference": 2,
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
        "owner": 3,
        "presentationKey": "p:3a5931ad51ecdf6873854a1562b56b7b7d9888e1e6cd777cea90ceacf680e03d",
        "reference": 15,
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
        "owner": 4,
        "presentationKey": "p:7bb70ab2da6c5989cf2183830c3e95b0abea7513b1852286bd2f39106fbee151",
        "reference": 17,
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
        "owner": 3,
        "presentationKey": "p:7ae22fcd56fc9009cdfdfd817523966cf1f17989a9d02364310b468f16612016",
        "reference": 16,
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
        "owner": 6,
        "presentationKey": "p:78bfd2cc2d6bd6c75e4be40c09264a60af60cead3aa234d06eb5e626f49e6cc5",
        "reference": 18,
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
        "name": "second",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:37b46cd0d9096946093c2c3d45872ffe7ac56aa456a8b663a5fb35d3cd8696fc",
        "reference": 10,
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
        "owner": 8,
        "presentationKey": "p:dfc8415b02acf0734bd6aea2500795a6cdbefe9dd866cb04139e46c9903614cb",
        "reference": 19,
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
        "owner": 9,
        "presentationKey": "p:f3c6557e13e607eeb0bc8e19d2209e1bb81b5023b42d487a90d69bb2c732d006",
        "reference": 21,
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
        "owner": 8,
        "presentationKey": "p:23fd0d5b9877bb0cd1cb3adccf6ef3b2c2cef218b9c94825e722b9b1f67399f3",
        "reference": 20,
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
        "owner": 11,
        "presentationKey": "p:98361ecd6b41052b5575d30194489c641af81c03c33a0637191efe503a99e39e",
        "reference": 22,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "occurrences",
            "members": [
              14,
              15
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "left",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:cbace341a2584867383e97681a47d4188912baf40b435d936ec0aca8a20fd224",
        "reference": 4,
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
        "metaclass": "OccurrenceUsage",
        "multiplicity": null,
        "name": "send",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:584906fc1897d61bfe44c4765fdc769289f0c43a29f5341509f91c36d37be1fb",
        "reference": 6,
        "source": 2,
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
        "metaclass": "OccurrenceUsage",
        "multiplicity": null,
        "name": "receive",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:bbb56f525823805ff9b139ea46a45c5f0fb8653915b68731d5486b34cf671675",
        "reference": 5,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "occurrences",
            "members": [
              17,
              18
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "right",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:5e461feffe4edb3937d8a6bc01e7fe8bb4db31c02bb0ca513bb6da8f902ae5e1",
        "reference": 7,
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
        "metaclass": "OccurrenceUsage",
        "multiplicity": null,
        "name": "send",
        "notationRole": "usage",
        "owner": 16,
        "presentationKey": "p:a433d366433045bd04e19a940394b38dfbda0b56960fd6ef8ef4a6c13bfeacac",
        "reference": 9,
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
        "metaclass": "OccurrenceUsage",
        "multiplicity": null,
        "name": "receive",
        "notationRole": "usage",
        "owner": 16,
        "presentationKey": "p:c91e6788398471776b7a39593d099def845a65df2e88f202dce3d842d2554c77",
        "reference": 8,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 14,
        "provenance": "authored",
        "reference": 29,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 15,
        "provenance": "authored",
        "reference": 30,
        "source": 1,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 17,
        "provenance": "authored",
        "reference": 31,
        "source": 2,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 18,
        "provenance": "authored",
        "reference": 32,
        "source": 2,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 8,
        "provenance": "authored",
        "reference": 35,
        "source": 3,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 9,
        "provenance": "authored",
        "reference": 36,
        "source": 3,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 11,
        "provenance": "authored",
        "reference": 48,
        "source": 8,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 12,
        "provenance": "authored",
        "reference": 49,
        "source": 8,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        13,
        16
      ],
      "messages": [
        {
          "label": "first",
          "navigation": 7,
          "node": 3,
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "source": {
            "lifeline": 13,
            "status": "resolved"
          },
          "target": {
            "lifeline": 16,
            "status": "resolved"
          }
        },
        {
          "label": "second",
          "navigation": 10,
          "node": 8,
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "source": {
            "lifeline": 16,
            "status": "resolved"
          },
          "target": {
            "lifeline": 13,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
