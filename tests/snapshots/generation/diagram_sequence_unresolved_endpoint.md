# META
~~~ini
description=Sequence view preserves an unresolved message endpoint as typed incomplete state
type=generate
libraries=standard
plugin=repository:diagram
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a41872ef46dfc43c6bed88539a765119751c4156310da4e7770fe79bb5e12c93") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (path (named (kind package) (name "SequenceUnresolved")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "sender::sent")) (flowTarget (reference "receiver::missing")))))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver::received"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver::received"))) (target (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::delivery")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction::receiver")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_unresolved_endpoint.md") (qualified-name "SequenceUnresolved::Interaction")))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:dc57bd1459da30b6a40d8f7d75718e7cfd2c32f1bb6e31ce002dc74ce760fd30",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_unresolved_endpoint.md",
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
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "flowSource",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "flowTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typeFeaturing",
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
      "ordinal": 11,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typeFeaturing",
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
      "ordinal": 0,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typeFeaturing",
      "source": 4
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
    }
  ],
  "selectedView": {
    "reference": 6,
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
        "origin": 2,
        "provenance": "authored",
        "reference": 13,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 3,
        "provenance": "authored",
        "reference": 28,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 1,
        "provenance": "authored",
        "reference": 14,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 4,
        "provenance": "authored",
        "reference": 15,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 5,
        "provenance": "authored",
        "reference": 23,
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
        2,
        4
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              2,
              4
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
        "metaclass": "FlowUsage",
        "name": "delivery",
        "notationRole": "usage",
        "owner": 0,
        "reference": 1,
        "source": 5,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "occurrences",
            "members": [
              3
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "metaclass": "PartUsage",
        "name": "sender",
        "notationRole": "usage",
        "owner": 0,
        "reference": 4,
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
        "name": "sent",
        "notationRole": "usage",
        "owner": 2,
        "reference": 5,
        "source": 2,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "occurrences",
            "members": [
              5
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "metaclass": "PartUsage",
        "name": "receiver",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 3,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "OccurrenceUsage",
        "name": "received",
        "notationRole": "usage",
        "owner": 4,
        "reference": 3,
        "source": 4,
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
        "reference": 29,
        "source": 2,
        "target": {
          "reference": 10,
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
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 31,
        "source": 3,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 32,
        "source": 3,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 6,
        "provenance": "authored",
        "reference": 16,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 7,
        "provenance": "authored",
        "reference": 17,
        "source": 1,
        "target": {
          "status": "unresolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 18,
        "source": 1,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 19,
        "source": 1,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 20,
        "source": 1,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 21,
        "source": 1,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 22,
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
        "reference": 24,
        "source": 4,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 25,
        "source": 4,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 26,
        "source": 5,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 27,
        "source": 5,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        2,
        4
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
            "lifeline": 2,
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
