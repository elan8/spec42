# META
~~~ini
description=Issue 286: an event cycle involving only one transfer stays explicitly unordered
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_single_message_event_cycle.md
viewQualifiedName=SingleCycle::seq
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package SingleCycle {
    part def Participant;
    occurrence def Scenario {
        part p : Participant { event occurrence sendEvent; }
        part q : Participant { event occurrence receiveEvent; }
        message only from p.sendEvent to q.receiveEvent;
        succession first q.receiveEvent then p.sendEvent;
    }
    view seq : StandardViewDefinitions::SequenceView { expose Scenario; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:863b17eda8051a914c31866021ee3f3edc1435895c2b056fcc1043c0b30cf7d8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "q::receiveEvent")) (succession (reference "p::sendEvent")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "p::sendEvent")) (flowTarget (reference "q::receiveEvent")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Participant")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Participant")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Scenario")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "q::receiveEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "p::sendEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (kind flowSource) (ordinal 0))
      (authored-target "p::sendEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (kind flowTarget) (ordinal 0))
      (authored-target "q::receiveEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Participant")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Participant")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent"))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "only")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 6 25) (end 6 39)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "q::receiveEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 6 45) (end 6 56)) (probe (position 6 45))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "p::sendEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 5 26) (end 5 37)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (kind flowSource) (ordinal 0) (authored-target "p::sendEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p::sendEvent")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 5 41) (end 5 55)) (probe (position 5 41))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::only"))) (kind flowTarget) (ordinal 0) (authored-target "q::receiveEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q::receiveEvent")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 3 17) (end 3 28)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::p"))) (kind featureTyping) (ordinal 0) (authored-target "Participant")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 4 17) (end 4 28)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario::q"))) (kind featureTyping) (ordinal 0) (authored-target "Participant")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Participant")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 8 15) (end 8 52)) (probe (position 8 15))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::seq"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (range (start 8 62) (end 8 70)) (probe (position 8 62))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (path (named (kind package) (name "SingleCycle")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_single_message_event_cycle.md") (qualified-name "SingleCycle::Scenario")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:c8bc067c157db8c3ffe9c909be215b18819b0e4b6f17b01837a8ae64f258d4fe",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_single_message_event_cycle.md",
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
        27
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        17,
        3,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        48,
        3,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        17,
        4,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        48,
        4,
        60
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        16,
        5,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        26,
        5,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        41,
        5,
        55
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        8,
        6,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        25,
        6,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        45,
        6,
        56
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        9,
        8,
        12
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Participant"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::only"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::only::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::only::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::p"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::p::sendEvent"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::q"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::Scenario::q::receiveEvent"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "SingleCycle::seq"
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
      "ownerQualifiedName": "SingleCycle::Scenario",
      "source": 10,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SingleCycle::Scenario::only",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "SingleCycle::Scenario::only",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SingleCycle::Scenario::only::",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "SingleCycle::Scenario::only::",
      "source": 9,
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
      "ordinal": 10,
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
      "ordinal": 29,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "flowSource",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "flowTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 20,
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
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "redefinition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "redefinition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "flow",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "succession",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typeFeaturing",
      "source": 9
    }
  ],
  "selectedView": {
    "reference": 10,
    "kind": "sequence-view",
    "name": "seq",
    "source": 13
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
        "origin": 7,
        "provenance": "authored",
        "reference": 28,
        "source": 0,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 8,
        "provenance": "authored",
        "reference": 57,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 9,
        "provenance": "authored",
        "reference": 29,
        "source": 0,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 10,
        "provenance": "authored",
        "reference": 65,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 2,
        "provenance": "authored",
        "reference": 30,
        "source": 0,
        "target": 2
      },
      {
        "kind": "flow",
        "navigation": 8,
        "origin": 2,
        "provenance": "implied",
        "reference": 62,
        "source": 8,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 3,
        "provenance": "authored",
        "reference": 37,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 4,
        "provenance": "authored",
        "reference": 47,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 5,
        "provenance": "authored",
        "reference": 38,
        "source": 2,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 6,
        "provenance": "authored",
        "reference": 48,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 1,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 11,
        "origin": 1,
        "provenance": "implied",
        "reference": 71,
        "source": 10,
        "target": 8
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        2
      ],
      "participants": [
        7,
        9
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              7,
              9
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              2
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "OccurrenceDefinition",
        "name": "Scenario",
        "notationRole": "definition",
        "owner": null,
        "reference": 1,
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
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 23,
        "source": 10,
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
        "name": "only",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
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
        "metaclass": "FlowEnd",
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "reference": 24,
        "source": 8,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 3,
        "reference": 26,
        "source": 8,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "reference": 25,
        "source": 9,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 5,
        "reference": 27,
        "source": 9,
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
              8
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "p",
        "notationRole": "usage",
        "owner": 0,
        "reference": 6,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Participant",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "OccurrenceUsage",
        "name": "sendEvent",
        "notationRole": "usage",
        "owner": 7,
        "reference": 7,
        "source": 3,
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
              10
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "q",
        "notationRole": "usage",
        "owner": 0,
        "reference": 8,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Participant",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "OccurrenceUsage",
        "name": "receiveEvent",
        "notationRole": "usage",
        "owner": 9,
        "reference": 9,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
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
        "reference": 32,
        "source": 0,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 61,
        "source": 7,
        "target": {
          "reference": 0,
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
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 59,
        "source": 7,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 60,
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
        "reference": 63,
        "source": 8,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 64,
        "source": 8,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 69,
        "source": 9,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 66,
        "source": 9,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 67,
        "source": 9,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 68,
        "source": 9,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 70,
        "source": 10,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 72,
        "source": 10,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 8,
        "provenance": "authored",
        "reference": 39,
        "source": 2,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 9,
        "provenance": "authored",
        "reference": 40,
        "source": 2,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 41,
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
        "reference": 42,
        "source": 2,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 43,
        "source": 2,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 44,
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
        "reference": 45,
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
        "reference": 46,
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
        "reference": 49,
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
        "reference": 51,
        "source": 4,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 53,
        "source": 4,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 55,
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
        "reference": 50,
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
        "reference": 52,
        "source": 6,
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
        "source": 6,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 56,
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
        "reference": 33,
        "source": 1,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 11,
        "provenance": "authored",
        "reference": 34,
        "source": 1,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 12,
        "provenance": "authored",
        "reference": 35,
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
        "reference": 36,
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
        7,
        9
      ],
      "messages": [
        {
          "label": "only",
          "navigation": 7,
          "node": 2,
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "source": {
            "lifeline": 7,
            "status": "resolved"
          },
          "target": {
            "lifeline": 9,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
