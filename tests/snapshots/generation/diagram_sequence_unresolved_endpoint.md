# META
~~~ini
description=Sequence view preserves an unresolved message endpoint as typed incomplete state
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_unresolved_endpoint.md
viewQualifiedName=SequenceUnresolved::selected
~~~
# SOURCE
~~~sysml
package SequenceUnresolved {
    private import StandardViewDefinitions::*;
    occurrence def Interaction {
        part sender { event occurrence sent; }
        part receiver { event occurrence received; }
        message delivery from sender.sent to receiver.missing;
    }
    view selected : SequenceView { expose Interaction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 45) (end 5 61))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b7491928848194a87b7f879f19171573973e651dab851c2ba681e7fb71276484") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "sender::sent")) (flowTarget (reference "receiver::missing")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver::received"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender::sent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Interaction")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (kind flowSource) (ordinal 0))
      (authored-target "sender::sent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender::sent")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (kind flowTarget) (ordinal 0))
      (authored-target "receiver::missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender::sent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver::received"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver::received"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender::sent"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender::sent"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "delivery")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver::received")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender::sent")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (range (start 5 30) (end 5 41)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (kind flowSource) (ordinal 0) (authored-target "sender::sent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::sender::sent")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (range (start 5 45) (end 5 61)) (probe (position 5 45))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (kind flowTarget) (ordinal 0) (authored-target "receiver::missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (range (start 7 20) (end 7 32)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (range (start 7 42) (end 7 53)) (probe (position 7 42))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:4b801f15c918daa66be1e735d939eab799878f74930ae00bb163c8eb039f0370",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_unresolved_endpoint.md",
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
        19
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        39,
        3,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        41,
        4,
        49
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
        61
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::Interaction"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::Interaction::delivery"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::Interaction::delivery::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::Interaction::receiver"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::Interaction::receiver::received"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::Interaction::sender"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::Interaction::sender::sent"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceUnresolved::selected"
    },
    {
      "kind": "source-anchor",
      "source": 6,
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 7,
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 6,
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 7,
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery::",
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
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "flowSource",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "flowTarget",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "containment",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 1
    }
  ],
  "selectedView": {
    "reference": 7,
    "kind": "sequence-view",
    "name": "selected",
    "source": 8
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "relationship-unresolved",
        "relationshipKind": "flowTarget"
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
              6,
              8
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              1
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
        "reference": 1,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "delivery",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
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
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 10,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 2,
        "source": 6,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 9,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 7,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 11,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 4,
        "source": 7,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "sender",
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
              7
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
        "name": "sent",
        "typing": {
          "status": "absent"
        },
        "owner": 6,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 3,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "receiver",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 3,
        "compartments": [
          {
            "kind": "occurrences",
            "provenance": "direct",
            "members": [
              9
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
        "reference": 4,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "received",
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 4,
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
        "reference": 17,
        "source": 1,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 7
        },
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 18,
        "source": 1,
        "kind": "flowTarget",
        "target": {
          "status": "unresolved"
        },
        "provenance": "authored",
        "navigation": 7
      }
    ],
    "edges": [
      {
        "reference": 12,
        "source": 0,
        "target": 6,
        "origin": 6,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 22,
        "source": 6,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 13,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 15,
        "source": 1,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 19,
        "source": 2,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 16,
        "source": 1,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 20,
        "source": 4,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 14,
        "source": 0,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 21,
        "source": 8,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 4
      }
    ],
    "metadata": {
      "participants": [
        6,
        8
      ],
      "messages": [
        1
      ]
    },
    "scene": {
      "kind": "sequence",
      "lifelines": [
        6,
        8
      ],
      "messages": [
        {
          "node": 1,
          "label": "delivery",
          "source": {
            "status": "resolved",
            "lifeline": 6
          },
          "target": {
            "status": "unresolved"
          },
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "navigation": 5
        }
      ]
    }
  }
}

~~~
