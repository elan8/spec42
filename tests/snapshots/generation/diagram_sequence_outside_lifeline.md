# META
~~~ini
description=Sequence view preserves a resolved message endpoint outside its lifeline set as typed incomplete state
type=generate
libraries=standard
plugin=repository:diagram
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1b2fcd651c8c10cf6d548cba85dd5be41742d05621487544adc5c80d402f2695") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (path (named (kind package) (name "SequenceOutside")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "sender::sent")) (flowTarget (reference "SequenceOutside::external::received")))))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::delivery")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction::sender")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_outside_lifeline.md") (qualified-name "SequenceOutside::Interaction")))
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
  "schemaVersion": 5,
  "modelDigest": "blake3:ec4bfe005a119510bb3be507ea6a024845d8f0731e680878c8159ff358f36336",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_outside_lifeline.md",
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
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typeFeaturing",
      "source": 3
    }
  ],
  "selectedView": {
    "reference": 5,
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
        "origin": 2,
        "provenance": "authored",
        "reference": 12,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 3,
        "provenance": "authored",
        "reference": 21,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 1,
        "provenance": "authored",
        "reference": 13,
        "source": 0,
        "target": 1
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
        2
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              2
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
        "metaclass": "FlowUsage",
        "name": "delivery",
        "notationRole": "usage",
        "owner": 0,
        "reference": 1,
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
              3
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartUsage",
        "name": "sender",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "sent",
        "notationRole": "usage",
        "owner": 2,
        "reference": 3,
        "source": 2,
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
        "reference": 22,
        "source": 2,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 23,
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
        "reference": 24,
        "source": 3,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 25,
        "source": 3,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 4,
        "provenance": "authored",
        "reference": 14,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 5,
        "provenance": "authored",
        "reference": 15,
        "source": 1,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 16,
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
        "reference": 17,
        "source": 1,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 18,
        "source": 1,
        "target": {
          "reference": 11,
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
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 20,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        2
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
            "lifeline": 2,
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
