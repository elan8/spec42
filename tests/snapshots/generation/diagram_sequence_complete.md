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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:03b619897c9afce7c1f7d5a977486e9251141ddbea20f6f33e674ca4d10bb01b") (admitted (standard-library 94)))
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
  "schemaVersion": 6,
  "modelDigest": "blake3:1305ab8e32f2eccbe742e913878b734b850489f3745e692b36e443ef43bea64f",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_complete.md",
      "sourceDomain": "workspace"
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Client"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::call"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::call::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::client"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::client::reply"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::client::request"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::result"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::result::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::server"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Interaction::server::serve"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::Server"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SequenceExample::selected"
    },
    {
      "kind": "source-anchor",
      "source": 14,
      "ownerQualifiedName": "SequenceExample::Interaction",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 9,
      "ownerQualifiedName": "SequenceExample::Interaction::call",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 10,
      "ownerQualifiedName": "SequenceExample::Interaction::call",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 9,
      "ownerQualifiedName": "SequenceExample::Interaction::call::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 10,
      "ownerQualifiedName": "SequenceExample::Interaction::call::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 12,
      "ownerQualifiedName": "SequenceExample::Interaction::result",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 13,
      "ownerQualifiedName": "SequenceExample::Interaction::result",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 12,
      "ownerQualifiedName": "SequenceExample::Interaction::result::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 13,
      "ownerQualifiedName": "SequenceExample::Interaction::result::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "succession",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "succession",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "containment",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "flowSource",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "flowTarget",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "succession",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 37
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "flow",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "succession",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "containment",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "containment",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "flowSource",
      "ordinal": 21
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "flowTarget",
      "ordinal": 22
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "containment",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "typing",
      "ordinal": 45
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "flow",
      "ordinal": 9
    }
  ],
  "selectedView": {
    "reference": 13,
    "kind": "sequence-view",
    "name": "selected",
    "source": 17
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "kind": "sequence-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 1,
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
              12,
              15
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              2,
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
        "reference": 14,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 14,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 3,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "call",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 8,
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
        "owner": 2,
        "source": 9,
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
        "reference": 16,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 2,
        "source": 10,
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
        "owner": 5,
        "source": 10,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "result",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 11,
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
        "owner": 7,
        "source": 12,
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
        "reference": 20,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 7,
        "source": 13,
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
        "owner": 10,
        "source": 13,
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
        "name": "client",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Client"
            }
          ]
        },
        "owner": 0,
        "source": 1,
        "compartments": [
          {
            "kind": "occurrences",
            "provenance": "direct",
            "members": [
              13,
              14
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
        "name": "reply",
        "typing": {
          "status": "absent"
        },
        "owner": 12,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 7,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "request",
        "typing": {
          "status": "absent"
        },
        "owner": 12,
        "source": 3,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 10,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "server",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 12,
              "label": "Server"
            }
          ]
        },
        "owner": 0,
        "source": 5,
        "compartments": [
          {
            "kind": "occurrences",
            "provenance": "direct",
            "members": [
              16
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
        "reference": 11,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "serve",
        "typing": {
          "status": "absent"
        },
        "owner": 15,
        "source": 7,
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
        "reference": 32,
        "source": 2,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 14
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 33,
        "source": 2,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 16
        },
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 28,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 2
        },
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 29,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 7
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 44,
        "source": 7,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 16
        },
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 45,
        "source": 7,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 13
        },
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 39,
        "source": 12,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 49,
        "source": 15,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 12
        },
        "provenance": "authored",
        "navigation": 6
      }
    ],
    "edges": [
      {
        "reference": 23,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 40,
        "source": 14,
        "target": 16,
        "origin": 2,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 9
      },
      {
        "reference": 30,
        "source": 2,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 35,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 31,
        "source": 2,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 36,
        "source": 5,
        "target": 6,
        "origin": 6,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 24,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 34,
        "source": 2,
        "target": 7,
        "origin": 1,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 15
      },
      {
        "reference": 25,
        "source": 0,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 50,
        "source": 16,
        "target": 13,
        "origin": 7,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 12
      },
      {
        "reference": 42,
        "source": 7,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 46,
        "source": 8,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 43,
        "source": 7,
        "target": 10,
        "origin": 10,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 47,
        "source": 10,
        "target": 11,
        "origin": 11,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 26,
        "source": 0,
        "target": 12,
        "origin": 12,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 37,
        "source": 12,
        "target": 13,
        "origin": 13,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 41,
        "source": 14,
        "target": 13,
        "origin": 13,
        "kind": "succession",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 38,
        "source": 12,
        "target": 14,
        "origin": 14,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 27,
        "source": 0,
        "target": 15,
        "origin": 15,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 48,
        "source": 15,
        "target": 16,
        "origin": 16,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      }
    ],
    "metadata": {
      "participants": [
        12,
        15
      ],
      "messages": [
        2,
        7
      ]
    },
    "scene": {
      "kind": "sequence",
      "lifelines": [
        12,
        15
      ],
      "messages": [
        {
          "node": 2,
          "label": "call",
          "source": {
            "status": "resolved",
            "lifeline": 12
          },
          "target": {
            "status": "resolved",
            "lifeline": 15
          },
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "navigation": 8
        },
        {
          "node": 7,
          "label": "result",
          "source": {
            "status": "resolved",
            "lifeline": 15
          },
          "target": {
            "status": "resolved",
            "lifeline": 12
          },
          "order": {
            "status": "resolved",
            "value": 2
          },
          "provenance": "authored",
          "navigation": 11
        }
      ]
    }
  }
}

~~~
