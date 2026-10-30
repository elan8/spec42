# META
~~~ini
description=Sequence view preserves a resolved message endpoint outside its lifeline set as typed incomplete state
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_outside_lifeline.md
viewQualifiedName=SequenceOutside::selected
~~~
# SOURCE
~~~sysml
package SequenceOutside {
    private import StandardViewDefinitions::*;
    part external { event occurrence received; }
    occurrence def Interaction {
        part sender { event occurrence sent; }
        message delivery from sender.sent to SequenceOutside::external::received;
    }
    view selected : SequenceView { expose Interaction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_outside_lifeline.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:51295c3371cf14667b45259f2fc18d7f8fb554a693075362e8eeb81a669411f2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "sender::sent")) (flowTarget (reference "SequenceOutside::external::received")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender::sent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external::received"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Interaction")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flowSource) (ordinal 0))
      (authored-target "sender::sent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender::sent")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flowTarget) (ordinal 0))
      (authored-target "SequenceOutside::external::received")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external::received")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender::sent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external::received"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender"))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender::sent"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender::sent"))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external::received"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external::received"))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender::sent")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external::received")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (range (start 5 30) (end 5 41)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flowSource) (ordinal 0) (authored-target "sender::sent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender::sent")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (range (start 5 45) (end 5 80)) (probe (position 5 45))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flowTarget) (ordinal 0) (authored-target "SequenceOutside::external::received")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::external::received")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (range (start 7 20) (end 7 32)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (range (start 7 42) (end 7 53)) (probe (position 7 42))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:bda3fb86f9985c91463b60301a1c2256986698bb1c1e39d381f1ac378f6b1591",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_outside_lifeline.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        19,
        3,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        19
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
        5,
        16,
        5,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        30,
        5,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        45,
        5,
        80
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        9,
        7,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceOutside::Interaction"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceOutside::Interaction::delivery"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceOutside::Interaction::delivery::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceOutside::Interaction::sender"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceOutside::Interaction::sender::sent"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceOutside::external::received"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceOutside::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceOutside::Interaction::delivery",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceOutside::Interaction::delivery",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceOutside::Interaction::delivery::",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceOutside::Interaction::delivery::",
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
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "flowSource",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "flowTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 3
    }
  ],
  "selectedView": {
    "reference": 6,
    "kind": "sequence-view",
    "name": "selected",
    "source": 6
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "sequence-message-endpoint-outside-lifeline"
      }
    ]
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 6,
        "provenance": "authored",
        "reference": 11,
        "source": 0,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 7,
        "provenance": "authored",
        "reference": 19,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 1,
        "provenance": "authored",
        "reference": 12,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 2,
        "provenance": "authored",
        "reference": 13,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 3,
        "provenance": "authored",
        "reference": 17,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 4,
        "provenance": "authored",
        "reference": 14,
        "source": 1,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 5,
        "provenance": "authored",
        "reference": 18,
        "source": 4,
        "target": 5
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        1
      ],
      "participants": [
        6
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              6
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1
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
        "presentationKey": "p:babdb8e32b45b80fc2299ce039b4227f8fdf5498733037aeca87b508da73ad2c",
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
        "metaclass": "FlowUsage",
        "multiplicity": null,
        "name": "delivery",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:deeb629b6a7c74e4a40724af6565c699b5b35ef09f26e0c5cb8741a5e68bee46",
        "reference": 1,
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
        "owner": 1,
        "presentationKey": "p:2ee845c4848e4dd45eb0be8af0ed63496c1eacaa28e90160ab770e2c9d70bcc6",
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "presentationKey": "p:6bc5aa92da8b5ecd54388a52d2f6c9efa704a5394c13bbe768079b8cbfc885d1",
        "reference": 9,
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
        "owner": 1,
        "presentationKey": "p:2297a575fd08e056ad078ed06eb7de94329cc66e02cdf19619c023baa001182b",
        "reference": 8,
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
        "owner": 4,
        "presentationKey": "p:303bbf931656f062fcb7edd95774243d1c8055de9bc080d10c96c8e0cfd6e084",
        "reference": 10,
        "source": 5,
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
              7
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "sender",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:6df0ec8c40b9d7d8e93287f2aeb56fda92ff177215c5f4fac7cc3bc19fc5a3e0",
        "reference": 3,
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
        "name": "sent",
        "notationRole": "usage",
        "owner": 6,
        "presentationKey": "p:07fa0ed4f25045185c8131d91774ca535633b1be5b4e75645c8154f864d85bee",
        "reference": 4,
        "source": 2,
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
        "navigation": 4,
        "provenance": "authored",
        "reference": 15,
        "source": 1,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 5,
        "provenance": "authored",
        "reference": 16,
        "source": 1,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        6
      ],
      "messages": [
        {
          "label": "delivery",
          "navigation": 3,
          "node": 1,
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "source": {
            "lifeline": 6,
            "status": "resolved"
          },
          "target": {
            "status": "outside-lifeline"
          }
        }
      ]
    }
  }
}

~~~
