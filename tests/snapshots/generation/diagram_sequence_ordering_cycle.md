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
  "schemaVersion": 6,
  "modelDigest": "blake3:d090175ffdeff652c3001a4de3dadaf36a1b814ce93d488cd6fd3d5e08fc9646",
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::first"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::first::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::left"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::left::receive"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::left::send"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::right"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::right::receive"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::right::send"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::second"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::Interaction::second::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceCycle::selected"
    },
    {
      "kind": "source-anchor",
      "source": 13,
      "ownerQualifiedName": "SequenceCycle::Interaction",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 16,
      "ownerQualifiedName": "SequenceCycle::Interaction",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 8,
      "ownerQualifiedName": "SequenceCycle::Interaction::first",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 9,
      "ownerQualifiedName": "SequenceCycle::Interaction::first",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 8,
      "ownerQualifiedName": "SequenceCycle::Interaction::first::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 9,
      "ownerQualifiedName": "SequenceCycle::Interaction::first::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 11,
      "ownerQualifiedName": "SequenceCycle::Interaction::second",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 12,
      "ownerQualifiedName": "SequenceCycle::Interaction::second",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 11,
      "ownerQualifiedName": "SequenceCycle::Interaction::second::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 12,
      "ownerQualifiedName": "SequenceCycle::Interaction::second::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "succession",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "succession",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "succession",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "succession",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "flowSource",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "flowTarget",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "succession",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "containment",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "containment",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "flow",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "containment",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "containment",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "flow",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 20
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "flowSource",
      "ordinal": 39
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "flowTarget",
      "ordinal": 40
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "succession",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "containment",
      "ordinal": 21
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
        "code": "sequence-ordering-cycle"
      }
    ]
  },
  "projection": {
    "kind": "sequence-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 0,
        "metaclass": "OccurrenceDefinition",
        "notationRole": "definition",
        "name": "Interaction",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 0,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "direct",
            "members": [
              13,
              16
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              3,
              8
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 13,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 13,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 14,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 16,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "first",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 7,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 15,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 17,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 4,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 16,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 9,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 18,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 6,
        "source": 9,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 10,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "second",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 10,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 19,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 11,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 21,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 9,
        "source": 11,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 20,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 12,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 22,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 11,
        "source": 12,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 4,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "left",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 1,
        "compartments": [
          {
            "kind": "occurrences",
            "provenance": "direct",
            "members": [
              14,
              15
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 6,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "send",
        "typing": {
          "status": "absent"
        },
        "owner": 13,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "receive",
        "typing": {
          "status": "absent"
        },
        "owner": 13,
        "source": 3,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 7,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "right",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 4,
        "compartments": [
          {
            "kind": "occurrences",
            "provenance": "direct",
            "members": [
              17,
              18
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 9,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "send",
        "typing": {
          "status": "absent"
        },
        "owner": 16,
        "source": 5,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "receive",
        "typing": {
          "status": "absent"
        },
        "owner": 16,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 29,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 3
        },
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 30,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 8
        },
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 31,
        "source": 2,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 8
        },
        "provenance": "authored",
        "navigation": 17
      },
      {
        "reference": 32,
        "source": 2,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 3
        },
        "provenance": "authored",
        "navigation": 18
      },
      {
        "reference": 35,
        "source": 3,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 14
        },
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 36,
        "source": 3,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 18
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 48,
        "source": 8,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 49,
        "source": 8,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 15
        },
        "provenance": "authored",
        "navigation": 12
      }
    ],
    "edges": [
      {
        "reference": 23,
        "source": 0,
        "target": 13,
        "origin": 13,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 40,
        "source": 13,
        "target": 14,
        "origin": 14,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 41,
        "source": 13,
        "target": 15,
        "origin": 15,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 24,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 37,
        "source": 3,
        "target": 8,
        "origin": 1,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 14
      },
      {
        "reference": 25,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 50,
        "source": 8,
        "target": 3,
        "origin": 2,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 17
      },
      {
        "reference": 26,
        "source": 0,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 42,
        "source": 14,
        "target": 18,
        "origin": 3,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 33,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 38,
        "source": 4,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 34,
        "source": 3,
        "target": 6,
        "origin": 6,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 39,
        "source": 6,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 27,
        "source": 0,
        "target": 16,
        "origin": 16,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 43,
        "source": 16,
        "target": 17,
        "origin": 17,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 44,
        "source": 16,
        "target": 18,
        "origin": 18,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 28,
        "source": 0,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 45,
        "source": 17,
        "target": 15,
        "origin": 8,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 11
      },
      {
        "reference": 46,
        "source": 8,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 51,
        "source": 9,
        "target": 10,
        "origin": 10,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 47,
        "source": 8,
        "target": 11,
        "origin": 11,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 52,
        "source": 11,
        "target": 12,
        "origin": 12,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 12
      }
    ],
    "metadata": {
      "participants": [
        13,
        16
      ],
      "messages": [
        3,
        8
      ]
    },
    "scene": {
      "kind": "sequence",
      "lifelines": [
        13,
        16
      ],
      "messages": [
        {
          "node": 3,
          "label": "first",
          "source": {
            "status": "resolved",
            "lifeline": 13
          },
          "target": {
            "status": "resolved",
            "lifeline": 16
          },
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "navigation": 7
        },
        {
          "node": 8,
          "label": "second",
          "source": {
            "status": "resolved",
            "lifeline": 16
          },
          "target": {
            "status": "resolved",
            "lifeline": 13
          },
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "navigation": 10
        }
      ]
    }
  }
}

~~~
