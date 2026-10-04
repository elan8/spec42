# META
~~~ini
description=Sequence view projects authoritative participants and flow facts
type=generate
libraries=standard
plugin=native:diagram
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Client")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "server::serve")) (flowTarget (reference "client::reply")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::reply"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client::request"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::call")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "call")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::client")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
      (type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Client")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
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
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::result")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (path (named (kind package) (name "SequenceExample")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "result")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction::server")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Interaction")))
      (type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_complete.md") (qualified-name "SequenceExample::Server")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
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
  "modelDigest": "blake3:18789fa9043a66226bf9cc9869d1b38aa3a8f08fd34d82d8268c7a886f1bc28d",
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
      "qualifiedName": "SequenceExample::Interaction::call::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::call::::"
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
      "qualifiedName": "SequenceExample::Interaction::result::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SequenceExample::Interaction::result::::"
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
      "ownerQualifiedName": "SequenceExample::Interaction",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceExample::Interaction::call",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceExample::Interaction::call",
      "source": 10,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceExample::Interaction::call::",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceExample::Interaction::call::",
      "source": 10,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceExample::Interaction::result",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SequenceExample::Interaction::result",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceExample::Interaction::result::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SequenceExample::Interaction::result::",
      "source": 13,
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
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "flowSource",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "flowTarget",
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
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "redefinition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "redefinition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "flow",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "flowSource",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "flowTarget",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "subsetting",
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
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "typeFeaturing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "typeFeaturing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "redefinition",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "redefinition",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "flow",
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
      "ordinal": 50,
      "relationshipKind": "typeFeaturing",
      "source": 13
    }
  ],
  "selectedView": {
    "reference": 15,
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
        "reference": 37,
        "source": 0,
        "target": 2
      },
      {
        "kind": "flow",
        "navigation": 9,
        "origin": 2,
        "provenance": "implied",
        "reference": 76,
        "source": 14,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 3,
        "provenance": "authored",
        "reference": 47,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 4,
        "provenance": "authored",
        "reference": 58,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 5,
        "provenance": "authored",
        "reference": 48,
        "source": 2,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 6,
        "provenance": "authored",
        "reference": 59,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 1,
        "provenance": "authored",
        "reference": 38,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 15,
        "origin": 1,
        "provenance": "implied",
        "reference": 56,
        "source": 2,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 7,
        "provenance": "authored",
        "reference": 39,
        "source": 0,
        "target": 7
      },
      {
        "kind": "flow",
        "navigation": 12,
        "origin": 7,
        "provenance": "implied",
        "reference": 104,
        "source": 16,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 8,
        "provenance": "authored",
        "reference": 79,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 9,
        "provenance": "authored",
        "reference": 89,
        "source": 8,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 10,
        "provenance": "authored",
        "reference": 80,
        "source": 7,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 11,
        "provenance": "authored",
        "reference": 90,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 12,
        "provenance": "authored",
        "reference": 40,
        "source": 0,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 13,
        "provenance": "authored",
        "reference": 68,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 14,
        "provenance": "authored",
        "reference": 69,
        "source": 12,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 15,
        "provenance": "authored",
        "reference": 41,
        "source": 0,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 16,
        "provenance": "authored",
        "reference": 99,
        "source": 15,
        "target": 16
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        2,
        7
      ],
      "participants": [
        12,
        15
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              12,
              15
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              2,
              7
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
        "reference": 1,
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
        "reference": 28,
        "source": 14,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
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
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowEnd",
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "reference": 29,
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
        "owner": 3,
        "reference": 31,
        "source": 9,
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
        "owner": 2,
        "reference": 30,
        "source": 10,
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
        "owner": 5,
        "reference": 32,
        "source": 10,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "FlowUsage",
        "name": "result",
        "notationRole": "usage",
        "owner": 0,
        "reference": 9,
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
        "owner": 7,
        "reference": 33,
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
        "owner": 8,
        "reference": 35,
        "source": 12,
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
        "owner": 7,
        "reference": 34,
        "source": 13,
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
        "owner": 10,
        "reference": 36,
        "source": 13,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "occurrences",
            "members": [
              13,
              14
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "metaclass": "PartUsage",
        "name": "client",
        "notationRole": "usage",
        "owner": 0,
        "reference": 6,
        "source": 1,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Client",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "OccurrenceUsage",
        "name": "reply",
        "notationRole": "usage",
        "owner": 12,
        "reference": 7,
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
        "name": "request",
        "notationRole": "usage",
        "owner": 12,
        "reference": 8,
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
              16
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "metaclass": "PartUsage",
        "name": "server",
        "notationRole": "usage",
        "owner": 0,
        "reference": 12,
        "source": 5,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Server",
              "reference": 14
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "metaclass": "OccurrenceUsage",
        "name": "serve",
        "notationRole": "usage",
        "owner": 15,
        "reference": 13,
        "source": 7,
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
        "reference": 42,
        "source": 0,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 9,
        "provenance": "authored",
        "reference": 49,
        "source": 2,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 10,
        "provenance": "authored",
        "reference": 50,
        "source": 2,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 51,
        "source": 2,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 52,
        "source": 2,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 53,
        "source": 2,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 54,
        "source": 2,
        "target": {
          "reference": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 55,
        "source": 2,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 57,
        "source": 2,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 60,
        "source": 3,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 4,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 64,
        "source": 4,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 66,
        "source": 4,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 61,
        "source": 5,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 63,
        "source": 6,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 65,
        "source": 6,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 67,
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
        "source": 1,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 15,
        "provenance": "authored",
        "reference": 44,
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
        "reference": 45,
        "source": 1,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 46,
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
        "reference": 81,
        "source": 7,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 13,
        "provenance": "authored",
        "reference": 82,
        "source": 7,
        "target": {
          "node": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 83,
        "source": 7,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 84,
        "source": 7,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 85,
        "source": 7,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 86,
        "source": 7,
        "target": {
          "reference": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 87,
        "source": 7,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 88,
        "source": 7,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 91,
        "source": 8,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 93,
        "source": 9,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 95,
        "source": 9,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 97,
        "source": 9,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 92,
        "source": 10,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 94,
        "source": 11,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 96,
        "source": 11,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 98,
        "source": 11,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 73,
        "source": 12,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 70,
        "source": 12,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 71,
        "source": 12,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 72,
        "source": 12,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 74,
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
        "reference": 75,
        "source": 13,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 77,
        "source": 14,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 78,
        "source": 14,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 103,
        "source": 15,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 100,
        "source": 15,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 101,
        "source": 15,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 102,
        "source": 15,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 105,
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
        "reference": 106,
        "source": 16,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        12,
        15
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
            "lifeline": 12,
            "status": "resolved"
          },
          "target": {
            "lifeline": 15,
            "status": "resolved"
          }
        },
        {
          "label": "result",
          "navigation": 11,
          "node": 7,
          "order": {
            "status": "resolved",
            "value": 2
          },
          "provenance": "authored",
          "source": {
            "lifeline": 15,
            "status": "resolved"
          },
          "target": {
            "lifeline": 12,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
