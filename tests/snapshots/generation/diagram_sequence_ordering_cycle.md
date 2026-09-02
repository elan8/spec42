# META
~~~ini
description=Sequence view exposes cyclic message ordering as an incomplete typed scene
type=generate
libraries=standard
plugin=repository:diagram
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "right::send")) (flowTarget (reference "left::receive")))))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::receive"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left::send"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::receive"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right::send"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::right"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::second"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (path (named (kind package) (name "SequenceCycle")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::first")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction::left")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_ordering_cycle.md") (qualified-name "SequenceCycle::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
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
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_ordering_cycle.md",
      "sourceDomain": "workspace"
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
      "qualifiedName": "SequenceCycle::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Flows::flows"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Flows::messages"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::occurrences"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "Transfers::flowTransfers"
    },
    {
      "document": 4,
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
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "succession",
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
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
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
      "ordinal": 12,
      "relationshipKind": "flowSource",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "flowTarget",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 17,
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
      "ordinal": 18,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "flow",
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
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "flow",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "flowSource",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "flowTarget",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "typeFeaturing",
      "source": 9
    }
  ],
  "selectedView": {
    "reference": 10,
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
        "origin": 5,
        "provenance": "authored",
        "reference": 19,
        "source": 0,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 6,
        "provenance": "authored",
        "reference": 39,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 7,
        "provenance": "authored",
        "reference": 40,
        "source": 5,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 1,
        "provenance": "authored",
        "reference": 20,
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
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 2,
        "provenance": "authored",
        "reference": 21,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 17,
        "origin": 2,
        "provenance": "implied",
        "reference": 63,
        "source": 4,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 3,
        "provenance": "authored",
        "reference": 22,
        "source": 0,
        "target": 3
      },
      {
        "kind": "flow",
        "navigation": 8,
        "origin": 3,
        "provenance": "implied",
        "reference": 45,
        "source": 6,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 8,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 9,
        "provenance": "authored",
        "reference": 48,
        "source": 8,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 10,
        "provenance": "authored",
        "reference": 49,
        "source": 8,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 4,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 4
      },
      {
        "kind": "flow",
        "navigation": 11,
        "origin": 4,
        "provenance": "implied",
        "reference": 54,
        "source": 9,
        "target": 7
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        3,
        4
      ],
      "participants": [
        5,
        8
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              5,
              8
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              3,
              4
            ],
            "provenance": "direct"
          }
        ],
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
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 17,
        "source": 13,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 18,
        "source": 16,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
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
        "metaclass": "FlowUsage",
        "name": "second",
        "notationRole": "usage",
        "owner": 0,
        "reference": 9,
        "source": 10,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "occurrences",
            "members": [
              6,
              7
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartUsage",
        "name": "left",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "send",
        "notationRole": "usage",
        "owner": 5,
        "reference": 5,
        "source": 2,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "receive",
        "notationRole": "usage",
        "owner": 5,
        "reference": 4,
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
              9,
              10
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartUsage",
        "name": "right",
        "notationRole": "usage",
        "owner": 0,
        "reference": 6,
        "source": 4,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "send",
        "notationRole": "usage",
        "owner": 8,
        "reference": 8,
        "source": 5,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "receive",
        "notationRole": "usage",
        "owner": 8,
        "reference": 7,
        "source": 6,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 41,
        "source": 5,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 42,
        "source": 5,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 46,
        "source": 6,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 47,
        "source": 6,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 43,
        "source": 7,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 44,
        "source": 7,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 14,
        "provenance": "authored",
        "reference": 25,
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
        "reference": 26,
        "source": 1,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 29,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 17,
        "provenance": "authored",
        "reference": 27,
        "source": 2,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 18,
        "provenance": "authored",
        "reference": 28,
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
        "reference": 30,
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
        "reference": 31,
        "source": 3,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 9,
        "provenance": "authored",
        "reference": 32,
        "source": 3,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
        "source": 3,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 34,
        "source": 3,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 35,
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
        "reference": 36,
        "source": 3,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 38,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 50,
        "source": 8,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 51,
        "source": 8,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 55,
        "source": 9,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 56,
        "source": 9,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 52,
        "source": 10,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 53,
        "source": 10,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 11,
        "provenance": "authored",
        "reference": 57,
        "source": 4,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 12,
        "provenance": "authored",
        "reference": 58,
        "source": 4,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 59,
        "source": 4,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 60,
        "source": 4,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 61,
        "source": 4,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 4,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 64,
        "source": 4,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        5,
        8
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
            "lifeline": 5,
            "status": "resolved"
          },
          "target": {
            "lifeline": 8,
            "status": "resolved"
          }
        },
        {
          "label": "second",
          "navigation": 10,
          "node": 4,
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "source": {
            "lifeline": 8,
            "status": "resolved"
          },
          "target": {
            "lifeline": 5,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
