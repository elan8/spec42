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
  "schemaVersion": 11,
  "modelDigest": "blake3:d13f8d91d6eadf783e262d4dfc5aa7cb69c0f8c9d82fda80240622f35984a29a",
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
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::Interaction"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::Interaction::delivery"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::Interaction::delivery::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::Interaction::receiver"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::Interaction::receiver::received"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::Interaction::sender"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::Interaction::sender::sent"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceUnresolved::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery",
      "source": 7,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery::",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceUnresolved::Interaction::delivery::",
      "source": 7,
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
      "ordinal": 7,
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
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 5
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
    "edges": [
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 6,
        "provenance": "authored",
        "reference": 12,
        "source": 0,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 7,
        "provenance": "authored",
        "reference": 22,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 1,
        "provenance": "authored",
        "reference": 13,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 2,
        "provenance": "authored",
        "reference": 15,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 3,
        "provenance": "authored",
        "reference": 19,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 4,
        "provenance": "authored",
        "reference": 16,
        "source": 1,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 5,
        "provenance": "authored",
        "reference": 20,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 8,
        "provenance": "authored",
        "reference": 14,
        "source": 0,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 9,
        "provenance": "authored",
        "reference": 21,
        "source": 8,
        "target": 9
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
        6,
        8
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              6,
              8
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
        "presentationKey": "p:e03d0347dada3ecb83664be4c803fd3eb2e71614ff77038f7e12487334aa6a5f",
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
        "presentationKey": "p:bc76643d8a626b94b8835e7e3c716f821330462582d98cc2c47e44faa8aa81fe",
        "reference": 1,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "presentationKey": "p:1dc56c0bd071425be240ab27ead056ad74ca83d943b1b6a06bd522405f4fc542",
        "reference": 8,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "presentationKey": "p:d68c39573daef4bb53e4e112c59a6a031d072eaf2eefe7953f35d1be7b73bd1a",
        "reference": 10,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "presentationKey": "p:36a672d622c1d55783e808fb16e3435d13422769b5e0c155b97ac1848ae21dd0",
        "reference": 9,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 4,
        "presentationKey": "p:d6cef02fcaf13e2a2a6f73441deb95b17bbd0d47479506fde9133d345d03e649",
        "reference": 11,
        "source": 7,
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
        "presentationKey": "p:5f70b387604db5907dc43259592bf6947fdc483fde5829517f21e3c98fcd9260",
        "reference": 5,
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
        "presentationKey": "p:188c867e5d3b10015be5d73cf786589c465537ca533d4f16ce2f303ce626778f",
        "reference": 6,
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
            "kind": "occurrences",
            "members": [
              9
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "receiver",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:2fb373abe1acc62741c50bae4466ac1d27e9a7ca1c4c237cf30f32fa638ac5c1",
        "reference": 3,
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
        "metaclass": "OccurrenceUsage",
        "multiplicity": null,
        "name": "received",
        "notationRole": "usage",
        "owner": 8,
        "presentationKey": "p:d0cb6ff19fe8dcc79bae650678737a45eb6027ad35532c7ffbfe785a1a3186d2",
        "reference": 4,
        "source": 4,
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
        "navigation": 6,
        "provenance": "authored",
        "reference": 17,
        "source": 1,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 7,
        "provenance": "authored",
        "reference": 18,
        "source": 1,
        "target": {
          "status": "unresolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        6,
        8
      ],
      "messages": [
        {
          "label": "delivery",
          "navigation": 5,
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
            "status": "unresolved"
          }
        }
      ]
    }
  }
}

~~~
