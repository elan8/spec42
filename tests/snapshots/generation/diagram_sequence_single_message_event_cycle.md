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
  "schemaVersion": 6,
  "modelDigest": "blake3:8bf1f9fdf8b96f102b4fa1efeed31dbbfb17d9b86669d0874951bdc7c9ea1a00",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_single_message_event_cycle.md",
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Participant"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario::only"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario::only::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario::p"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario::p::sendEvent"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario::q"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::Scenario::q::receiveEvent"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "SingleCycle::seq"
    },
    {
      "kind": "source-anchor",
      "source": 10,
      "ownerQualifiedName": "SingleCycle::Scenario",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 8,
      "ownerQualifiedName": "SingleCycle::Scenario::only",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 9,
      "ownerQualifiedName": "SingleCycle::Scenario::only",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 8,
      "ownerQualifiedName": "SingleCycle::Scenario::only::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 9,
      "ownerQualifiedName": "SingleCycle::Scenario::only::",
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
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "succession",
      "ordinal": 30
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "succession",
      "ordinal": 31
    },
    {
      "kind": "relationship",
      "source": 3,
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
      "source": 3,
      "relationshipKind": "flowSource",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "flowTarget",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "containment",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "flow",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "typing",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "succession",
      "ordinal": 11
    }
  ],
  "selectedView": {
    "reference": 9,
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
    "kind": "sequence-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 1,
        "metaclass": "OccurrenceDefinition",
        "notationRole": "definition",
        "name": "Scenario",
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
              7,
              9
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              2
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
        "reference": 10,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 10,
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
        "name": "only",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
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
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 2,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 13,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 12,
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
        "reference": 14,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 5,
        "source": 9,
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
        "name": "p",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Participant"
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
              8
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
        "name": "sendEvent",
        "typing": {
          "status": "absent"
        },
        "owner": 7,
        "source": 3,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 7,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "q",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Participant"
            }
          ]
        },
        "owner": 0,
        "source": 4,
        "compartments": [
          {
            "kind": "occurrences",
            "provenance": "direct",
            "members": [
              10
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
        "reference": 8,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "receiveEvent",
        "typing": {
          "status": "absent"
        },
        "owner": 9,
        "source": 6,
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
        "reference": 28,
        "source": 7,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 31,
        "source": 9,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 23,
        "source": 2,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 8
        },
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 24,
        "source": 2,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 10
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 19,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 10
        },
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 20,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 8
        },
        "provenance": "authored",
        "navigation": 12
      }
    ],
    "edges": [
      {
        "reference": 15,
        "source": 0,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 27,
        "source": 7,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 16,
        "source": 0,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 30,
        "source": 9,
        "target": 10,
        "origin": 10,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 17,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 29,
        "source": 8,
        "target": 10,
        "origin": 2,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 8
      },
      {
        "reference": 21,
        "source": 2,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 25,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 22,
        "source": 2,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 26,
        "source": 5,
        "target": 6,
        "origin": 6,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 18,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 32,
        "source": 10,
        "target": 8,
        "origin": 1,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 11
      }
    ],
    "metadata": {
      "participants": [
        7,
        9
      ],
      "messages": [
        2
      ]
    },
    "scene": {
      "kind": "sequence",
      "lifelines": [
        7,
        9
      ],
      "messages": [
        {
          "node": 2,
          "label": "only",
          "source": {
            "status": "resolved",
            "lifeline": 7
          },
          "target": {
            "status": "resolved",
            "lifeline": 9
          },
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "navigation": 7
        }
      ]
    }
  }
}

~~~
