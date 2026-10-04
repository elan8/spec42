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
        part left { event occurrence send; event occurrence receive; }
        part right { event occurrence send; event occurrence receive; }
        message first from left.send to right.receive;
        message second from right.send to left.receive;
        succession first first then second;
        succession first second then first;
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:70781c009e2cdd19dcec31d11adeaddc61a7878c5f7aef5ba6c1e254ce1dd9ce") (admitted (standard-library 94)))
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
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 7 25) (end 7 30)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 8 25) (end 8 31)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 7 36) (end 7 42)) (probe (position 7 36))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 8 37) (end 8 42)) (probe (position 8 37))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 5 27) (end 5 36)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowSource) (ordinal 0) (authored-target "left::send")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 5 40) (end 5 53)) (probe (position 5 40))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (kind flowTarget) (ordinal 0) (authored-target "right::receive")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 6 28) (end 6 38)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flowSource) (ordinal 0) (authored-target "right::send")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (range (start 6 42) (end 6 54)) (probe (position 6 42))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:7cf062814b4bef6a346232df8204d32895758f178da99e4d8ebf74ac4b9b8ba5",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_ordering_cycle.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/base.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/flows.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/occurrences.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/transfers.md",
      "sourceDomain": "standard-library"
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
        37,
        3,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        60,
        3,
        67
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
        38,
        4,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        61,
        4,
        68
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        16,
        5,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        27,
        5,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        40,
        5,
        53
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
        38
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        42,
        6,
        54
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        25,
        7,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        36,
        7,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        8,
        8,
        43
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
        42
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
      "qualifiedName": "SequenceCycle::Interaction::first::::"
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
      "qualifiedName": "SequenceCycle::Interaction::second::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceCycle::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Base::things"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Flows::flows"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Flows::messages"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::suboccurrences"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::happensBeforeLinks"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::occurrences"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "Transfers::Transfer::source::sourceOutput"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "Transfers::Transfer::target::targetInput"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "Transfers::flowTransfers"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "Transfers::transfers"
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
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "subsetting",
      "source": 1
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
      "ordinal": 11,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "typeFeaturing",
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
      "ordinal": 18,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "subsetting",
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
      "ordinal": 23,
      "relationshipKind": "typeFeaturing",
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
      "ordinal": 24,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "flow",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "typeFeaturing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "flow",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "typeFeaturing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "flowSource",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "flowTarget",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "redefinition",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "redefinition",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "typeFeaturing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "typeFeaturing",
      "source": 13
    }
  ],
  "selectedView": {
    "reference": 14,
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
    "edges": [
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 13,
        "provenance": "authored",
        "reference": 37,
        "source": 0,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 14,
        "provenance": "authored",
        "reference": 73,
        "source": 13,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 15,
        "provenance": "authored",
        "reference": 74,
        "source": 13,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 1,
        "provenance": "authored",
        "reference": 38,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 1,
        "provenance": "implied",
        "reference": 61,
        "source": 3,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 2,
        "provenance": "authored",
        "reference": 39,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 17,
        "origin": 2,
        "provenance": "implied",
        "reference": 102,
        "source": 8,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 3,
        "provenance": "authored",
        "reference": 40,
        "source": 0,
        "target": 3
      },
      {
        "kind": "flow",
        "navigation": 8,
        "origin": 3,
        "provenance": "implied",
        "reference": 80,
        "source": 14,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 4,
        "provenance": "authored",
        "reference": 52,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 5,
        "provenance": "authored",
        "reference": 63,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 6,
        "provenance": "authored",
        "reference": 53,
        "source": 3,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 7,
        "provenance": "authored",
        "reference": 64,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 16,
        "provenance": "authored",
        "reference": 41,
        "source": 0,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 17,
        "provenance": "authored",
        "reference": 83,
        "source": 16,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 18,
        "provenance": "authored",
        "reference": 84,
        "source": 16,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 8,
        "provenance": "authored",
        "reference": 42,
        "source": 0,
        "target": 8
      },
      {
        "kind": "flow",
        "navigation": 11,
        "origin": 8,
        "provenance": "implied",
        "reference": 90,
        "source": 17,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 9,
        "provenance": "authored",
        "reference": 93,
        "source": 8,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 10,
        "provenance": "authored",
        "reference": 104,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 11,
        "provenance": "authored",
        "reference": 94,
        "source": 8,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 12,
        "provenance": "authored",
        "reference": 105,
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
        "metaclass": "OccurrenceDefinition",
        "name": "Interaction",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 0,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 27,
        "source": 13,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 28,
        "source": 16,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowUsage",
        "name": "first",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 7,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowEnd",
        "name": null,
        "notationRole": "unsupported",
        "owner": 3,
        "reference": 29,
        "source": 8,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "Feature",
        "name": null,
        "notationRole": "unsupported",
        "owner": 4,
        "reference": 31,
        "source": 8,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowEnd",
        "name": null,
        "notationRole": "unsupported",
        "owner": 3,
        "reference": 30,
        "source": 9,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "Feature",
        "name": null,
        "notationRole": "unsupported",
        "owner": 6,
        "reference": 32,
        "source": 9,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowUsage",
        "name": "second",
        "notationRole": "usage",
        "owner": 0,
        "reference": 11,
        "source": 10,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowEnd",
        "name": null,
        "notationRole": "unsupported",
        "owner": 8,
        "reference": 33,
        "source": 11,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "Feature",
        "name": null,
        "notationRole": "unsupported",
        "owner": 9,
        "reference": 35,
        "source": 11,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowEnd",
        "name": null,
        "notationRole": "unsupported",
        "owner": 8,
        "reference": 34,
        "source": 12,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "Feature",
        "name": null,
        "notationRole": "unsupported",
        "owner": 11,
        "reference": 36,
        "source": 12,
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
        "metaclass": "PartUsage",
        "name": "left",
        "notationRole": "usage",
        "owner": 0,
        "reference": 5,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "OccurrenceUsage",
        "name": "send",
        "notationRole": "usage",
        "owner": 13,
        "reference": 7,
        "source": 2,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "OccurrenceUsage",
        "name": "receive",
        "notationRole": "usage",
        "owner": 13,
        "reference": 6,
        "source": 3,
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
        "metaclass": "PartUsage",
        "name": "right",
        "notationRole": "usage",
        "owner": 0,
        "reference": 8,
        "source": 4,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "OccurrenceUsage",
        "name": "send",
        "notationRole": "usage",
        "owner": 16,
        "reference": 10,
        "source": 5,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "OccurrenceUsage",
        "name": "receive",
        "notationRole": "usage",
        "owner": 16,
        "reference": 9,
        "source": 6,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 43,
        "source": 0,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 75,
        "source": 13,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 76,
        "source": 13,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 77,
        "source": 13,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 81,
        "source": 14,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 82,
        "source": 14,
        "target": {
          "node": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 78,
        "source": 15,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 79,
        "source": 15,
        "target": {
          "node": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 44,
        "source": 1,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 14,
        "provenance": "authored",
        "reference": 46,
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
        "reference": 47,
        "source": 1,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 50,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 45,
        "source": 2,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 17,
        "provenance": "authored",
        "reference": 48,
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
        "reference": 49,
        "source": 2,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 51,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 8,
        "provenance": "authored",
        "reference": 54,
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
        "reference": 55,
        "source": 3,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 56,
        "source": 3,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 57,
        "source": 3,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 58,
        "source": 3,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 59,
        "source": 3,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 60,
        "source": 3,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 65,
        "source": 4,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 67,
        "source": 5,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 69,
        "source": 5,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 71,
        "source": 5,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 66,
        "source": 6,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 68,
        "source": 7,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 70,
        "source": 7,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 72,
        "source": 7,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 85,
        "source": 16,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 86,
        "source": 16,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 87,
        "source": 16,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 91,
        "source": 17,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 92,
        "source": 17,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 88,
        "source": 18,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 89,
        "source": 18,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 11,
        "provenance": "authored",
        "reference": 95,
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
        "reference": 96,
        "source": 8,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 97,
        "source": 8,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 98,
        "source": 8,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 99,
        "source": 8,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 100,
        "source": 8,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 101,
        "source": 8,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 103,
        "source": 8,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 106,
        "source": 9,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 108,
        "source": 10,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 110,
        "source": 10,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 112,
        "source": 10,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 107,
        "source": 11,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 109,
        "source": 12,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 111,
        "source": 12,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 113,
        "source": 12,
        "target": {
          "node": 11,
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
