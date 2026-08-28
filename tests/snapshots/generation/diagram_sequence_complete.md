# META
~~~ini
description=Sequence view projects authoritative participants and flow facts
type=generate
libraries=standard
plugin=repository:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_complete.md
viewQualifiedName=SequenceExample::selected
~~~
# SOURCE
~~~sysml
package SequenceExample {
    private import StandardViewDefinitions::*;
    part def Client;
    part def Server;
    occurrence def Interaction {
        part client : Client {
            event occurrence request;
            then event occurrence reply;
        }
        part server : Server {
            event occurrence serve;
        }
        message call from client.request to server.serve;
        message result from server.serve to client.reply;
        succession first call then result;
    }
    view selected : SequenceView { expose Interaction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_complete.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:548a818fd9c0d010e1ac169570217f623f9dd877f1a361cf0e40913a6a22c212") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "call")) (succession (reference "result")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "client::request")) (flowTarget (reference "server::serve")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Client")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "server::serve")) (flowTarget (reference "client::reply")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Server")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Interaction")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "call")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "result")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (kind flowSource) (ordinal 0))
      (authored-target "client::request")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (kind flowTarget) (ordinal 0))
      (authored-target "server::serve")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (kind featureTyping) (ordinal 0))
      (authored-target "Client")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flowSource) (ordinal 0))
      (authored-target "server::serve")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flowTarget) (ordinal 0))
      (authored-target "client::reply")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (kind featureTyping) (ordinal 0))
      (authored-target "Server")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
      (type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
      (type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 14 25) (end 14 29)) (probe (position 14 25))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "call")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 14 35) (end 14 41)) (probe (position 14 35))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "result")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 12 26) (end 12 40)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (kind flowSource) (ordinal 0) (authored-target "client::request")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 12 44) (end 12 56)) (probe (position 12 44))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (kind flowTarget) (ordinal 0) (authored-target "server::serve")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 5 22) (end 5 28)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (kind featureTyping) (ordinal 0) (authored-target "Client")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 13 28) (end 13 40)) (probe (position 13 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flowSource) (ordinal 0) (authored-target "server::serve")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server::serve")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 13 44) (end 13 56)) (probe (position 13 44))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flowTarget) (ordinal 0) (authored-target "client::reply")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 9 22) (end 9 28)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (kind featureTyping) (ordinal 0) (authored-target "Server")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 16 20) (end 16 32)) (probe (position 16 20))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_complete.md") (range (start 16 42) (end 16 53)) (probe (position 16 42))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:e679fa972504af605b029daa41ecf5e7897aa33ef814e151a89f4c8b2c68e9ea",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_complete.md",
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
      "uri": "memory://snapshot/sysml.library/items.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/objects.md",
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
        4,
        19,
        4,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        22,
        5,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        29,
        6,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        34,
        7,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        22,
        9,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        29,
        10,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        16,
        12,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        26,
        12,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        44,
        12,
        56
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        16,
        13,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        28,
        13,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        44,
        13,
        56
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        8,
        14,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        25,
        14,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        35,
        14,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        9,
        16,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Client"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::call"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::client"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::client::reply"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::client::request"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::result"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::server"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::server::serve"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Server"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Base::Anything"
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
      "qualifiedName": "Items::Item"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "Objects::Object"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::occurrences"
    },
    {
      "document": 6,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 6,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "document": 7,
      "kind": "qualified-name",
      "qualifiedName": "Transfers::flowTransfers"
    },
    {
      "document": 7,
      "kind": "qualified-name",
      "qualifiedName": "Transfers::transfers"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "SequenceExample::Interaction",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "flowSource",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "flowTarget",
      "source": 3
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
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "flow",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "flowSource",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "flowTarget",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "flow",
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
      "relationshipKind": "typeFeaturing",
      "source": 9
    }
  ],
  "selectedView": {
    "reference": 11,
    "kind": "sequence-view",
    "name": "selected",
    "source": 17
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 2,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 2
      },
      {
        "kind": "flow",
        "navigation": 9,
        "origin": 2,
        "provenance": "implied",
        "reference": 47,
        "source": 6,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 1,
        "provenance": "authored",
        "reference": 25,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 15,
        "origin": 1,
        "provenance": "implied",
        "reference": 38,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 3,
        "provenance": "authored",
        "reference": 26,
        "source": 0,
        "target": 3
      },
      {
        "kind": "flow",
        "navigation": 12,
        "origin": 3,
        "provenance": "implied",
        "reference": 61,
        "source": 8,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 4,
        "provenance": "authored",
        "reference": 27,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 5,
        "provenance": "authored",
        "reference": 40,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 6,
        "provenance": "authored",
        "reference": 41,
        "source": 4,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 7,
        "provenance": "authored",
        "reference": 28,
        "source": 0,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 8,
        "provenance": "authored",
        "reference": 57,
        "source": 7,
        "target": 8
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        2,
        3
      ],
      "participants": [
        4,
        7
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              4,
              7
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              2,
              3
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "OccurrenceDefinition",
        "name": "Interaction",
        "notationRole": "definition",
        "owner": null,
        "reference": 1,
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
        "reference": 23,
        "source": 14,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "FlowUsage",
        "name": "call",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 8,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "FlowUsage",
        "name": "result",
        "notationRole": "usage",
        "owner": 0,
        "reference": 7,
        "source": 11,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "occurrences",
            "members": [
              5,
              6
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartUsage",
        "name": "client",
        "notationRole": "usage",
        "owner": 0,
        "reference": 4,
        "source": 1,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Anything",
              "reference": 12
            },
            {
              "label": "Item",
              "reference": 15
            },
            {
              "label": "Part",
              "reference": 19
            },
            {
              "label": "Object",
              "reference": 16
            },
            {
              "label": "Client",
              "reference": 0
            },
            {
              "label": "Occurrence",
              "reference": 17
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "reply",
        "notationRole": "usage",
        "owner": 4,
        "reference": 5,
        "source": 4,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "request",
        "notationRole": "usage",
        "owner": 4,
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
              8
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartUsage",
        "name": "server",
        "notationRole": "usage",
        "owner": 0,
        "reference": 8,
        "source": 5,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Anything",
              "reference": 12
            },
            {
              "label": "Item",
              "reference": 15
            },
            {
              "label": "Part",
              "reference": 19
            },
            {
              "label": "Object",
              "reference": 16
            },
            {
              "label": "Server",
              "reference": 10
            },
            {
              "label": "Occurrence",
              "reference": 17
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "OccurrenceUsage",
        "name": "serve",
        "notationRole": "usage",
        "owner": 7,
        "reference": 9,
        "source": 7,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "flowSource",
        "navigation": 9,
        "provenance": "authored",
        "reference": 32,
        "source": 2,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 10,
        "provenance": "authored",
        "reference": 33,
        "source": 2,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 34,
        "source": 2,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 35,
        "source": 2,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 36,
        "source": 2,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 37,
        "source": 2,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 39,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 15,
        "provenance": "authored",
        "reference": 29,
        "source": 1,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 16,
        "provenance": "authored",
        "reference": 30,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 31,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 12,
        "provenance": "authored",
        "reference": 50,
        "source": 3,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 13,
        "provenance": "authored",
        "reference": 51,
        "source": 3,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 52,
        "source": 3,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 53,
        "source": 3,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 54,
        "source": 3,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 55,
        "source": 3,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 56,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 44,
        "source": 4,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 42,
        "source": 4,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 43,
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
        "reference": 45,
        "source": 5,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 46,
        "source": 5,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 48,
        "source": 6,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 49,
        "source": 6,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 60,
        "source": 7,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 58,
        "source": 7,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 59,
        "source": 7,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 8,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 63,
        "source": 8,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        4,
        7
      ],
      "messages": [
        {
          "label": "call",
          "navigation": 8,
          "node": 2,
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "source": {
            "lifeline": 4,
            "status": "resolved"
          },
          "target": {
            "lifeline": 7,
            "status": "resolved"
          }
        },
        {
          "label": "result",
          "navigation": 11,
          "node": 3,
          "order": {
            "status": "resolved",
            "value": 2
          },
          "provenance": "authored",
          "source": {
            "lifeline": 7,
            "status": "resolved"
          },
          "target": {
            "lifeline": 4,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
