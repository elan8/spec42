# META
~~~ini
description=Issue 286: message order follows event successions across lifelines despite names and declaration order
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_event_order_semantic_names.md
viewQualifiedName=Seq::seq
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package Seq {
    item def A; item def B; item def C;
    part def P; part def Q; part def R;
    occurrence def Scenario {
        part p : P { event occurrence s1; then event occurrence r3; }
        part q : Q { event occurrence r1; then event occurrence s2; then event occurrence s3; }
        part r : R { event occurrence r2; }
        message ackMsg of C from q.s3 to p.r3;
        message pressMsg of A from p.s1 to q.r1;
        message telemetryMsg of B from q.s2 to r.r2;
    }
    view seq : StandardViewDefinitions::SequenceView { expose Scenario; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:35a8df3ee9a1e1a30cc316eb1d0ed9bec1fa7ba0bdf77260ccd5b272a9bb3f86") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::A"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::B"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::C"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "q::s3")) (flowTarget (reference "p::r3")) (flowPayloadType (reference "C")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::r3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::s1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "p::s1")) (flowTarget (reference "q::r1")) (flowPayloadType (reference "A")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::r1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r::r2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "q::s2")) (flowTarget (reference "r::r2")) (flowPayloadType (reference "B")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Scenario")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowSource) (ordinal 0))
      (authored-target "q::s3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s3")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowTarget) (ordinal 0))
      (authored-target "p::r3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::r3")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::C")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowSource) (ordinal 0))
      (authored-target "p::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::s1")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowTarget) (ordinal 0))
      (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::r1")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::A")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowSource) (ordinal 0))
      (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s2")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowTarget) (ordinal 0))
      (authored-target "r::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r::r2")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::B")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::r3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::s1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::r1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r::r2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::A"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::B"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::C"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::r3"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::r3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::s1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::s1"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::r1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::r1"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s2"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s3"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r::r2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r::r2"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::A")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::B")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::C")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::r3")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::s1")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::r1")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s2")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s3")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r::r2")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 7 33) (end 7 37)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowSource) (ordinal 0) (authored-target "q::s3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s3")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 7 41) (end 7 45)) (probe (position 7 41))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowTarget) (ordinal 0) (authored-target "p::r3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::r3")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 7 26) (end 7 27)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowPayloadType) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::C")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 4 17) (end 4 18)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::P")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 8 35) (end 8 39)) (probe (position 8 35))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowSource) (ordinal 0) (authored-target "p::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::p::s1")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 8 43) (end 8 47)) (probe (position 8 43))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowTarget) (ordinal 0) (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::r1")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 8 28) (end 8 29)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowPayloadType) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::A")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 5 17) (end 5 18)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Q")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 6 17) (end 6 18)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::R")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 9 39) (end 9 43)) (probe (position 9 39))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowSource) (ordinal 0) (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::q::s2")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 9 47) (end 9 51)) (probe (position 9 47))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowTarget) (ordinal 0) (authored-target "r::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::r::r2")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 9 32) (end 9 33)) (probe (position 9 32))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowPayloadType) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::B")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 11 15) (end 11 52)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (range (start 11 62) (end 11 70)) (probe (position 11 62))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_semantic_names.md") (qualified-name "Seq::Scenario")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:324a7ebd8fbdd054a4c8b236fa43994aefc182933a48b66dc755bd512e466769",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_event_order_semantic_names.md",
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
        27
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
        18
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        38,
        4,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        64,
        4,
        66
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        17,
        5,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        38,
        5,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        64,
        5,
        66
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        90,
        5,
        92
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        13,
        6,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        17,
        6,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        38,
        6,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        16,
        7,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        26,
        7,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        33,
        7,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        41,
        7,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        16,
        8,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        28,
        8,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        35,
        8,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        43,
        8,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        16,
        9,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        32,
        9,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        39,
        9,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        47,
        9,
        51
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        9,
        11,
        12
      ]
    }
  ],
  "references": [
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::A"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::B"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::C"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::P"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Q"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::R"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::ackMsg"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::ackMsg::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::p"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::p::r3"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::p::s1"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::pressMsg"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::pressMsg::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::q"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::q::r1"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::q::s2"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::q::s3"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::r"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::r::r2"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::telemetryMsg"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::Scenario::telemetryMsg::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Seq::seq"
    },
    {
      "kind": "source-anchor",
      "source": 15,
      "ownerQualifiedName": "Seq::Scenario::ackMsg",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 16,
      "ownerQualifiedName": "Seq::Scenario::ackMsg",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 15,
      "ownerQualifiedName": "Seq::Scenario::ackMsg::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 16,
      "ownerQualifiedName": "Seq::Scenario::ackMsg::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Seq::Scenario::pressMsg",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 20,
      "ownerQualifiedName": "Seq::Scenario::pressMsg",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Seq::Scenario::pressMsg::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 20,
      "ownerQualifiedName": "Seq::Scenario::pressMsg::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 23,
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 24,
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg",
      "metaclass": "FlowEnd",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 23,
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 24,
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg::",
      "metaclass": "Feature",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "containment",
      "ordinal": 24
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "containment",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "containment",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "flowPayloadType",
      "ordinal": 25
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "flowSource",
      "ordinal": 26
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "flowTarget",
      "ordinal": 27
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "containment",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "containment",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "typing",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "flow",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "succession",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "containment",
      "ordinal": 20
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "containment",
      "ordinal": 22
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "flowPayloadType",
      "ordinal": 42
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "flowSource",
      "ordinal": 43
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "flowTarget",
      "ordinal": 44
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "containment",
      "ordinal": 21
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "containment",
      "ordinal": 23
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "containment",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "containment",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "typing",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "succession",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "flow",
      "ordinal": 25
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "succession",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 17,
      "relationshipKind": "flow",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 18,
      "relationshipKind": "containment",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 18,
      "relationshipKind": "typing",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 20,
      "relationshipKind": "containment",
      "ordinal": 26
    },
    {
      "kind": "relationship",
      "source": 20,
      "relationshipKind": "containment",
      "ordinal": 28
    },
    {
      "kind": "relationship",
      "source": 20,
      "relationshipKind": "flowPayloadType",
      "ordinal": 59
    },
    {
      "kind": "relationship",
      "source": 20,
      "relationshipKind": "flowSource",
      "ordinal": 60
    },
    {
      "kind": "relationship",
      "source": 20,
      "relationshipKind": "flowTarget",
      "ordinal": 61
    },
    {
      "kind": "relationship",
      "source": 21,
      "relationshipKind": "containment",
      "ordinal": 27
    },
    {
      "kind": "relationship",
      "source": 21,
      "relationshipKind": "containment",
      "ordinal": 29
    }
  ],
  "selectedView": {
    "reference": 22,
    "kind": "sequence-view",
    "name": "seq",
    "source": 25
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
        "reference": 6,
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
              16,
              19,
              23
            ]
          },
          {
            "kind": "connections",
            "provenance": "direct",
            "members": [
              1,
              6,
              11
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
        "reference": 20,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "telemetryMsg",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 21,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 31,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 23,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 33,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 2,
        "source": 23,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 32,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 24,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 34,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 4,
        "source": 24,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 7,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "ackMsg",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 13,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 23,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 6,
        "source": 15,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 25,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 7,
        "source": 15,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 24,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 6,
        "source": 16,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 26,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 9,
        "source": 16,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 12,
        "metaclass": "FlowUsage",
        "notationRole": "usage",
        "name": "pressMsg",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 17,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 27,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 11,
        "source": 19,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 29,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 12,
        "source": 19,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 28,
        "metaclass": "FlowEnd",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 11,
        "source": 20,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 30,
        "metaclass": "Feature",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 14,
        "source": 20,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 9,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "p",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 3,
              "label": "P"
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
              17,
              18
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
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "r3",
        "typing": {
          "status": "absent"
        },
        "owner": 16,
        "source": 4,
        "compartments": [],
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
        "name": "s1",
        "typing": {
          "status": "absent"
        },
        "owner": 16,
        "source": 3,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 14,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "q",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 4,
              "label": "Q"
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
              20,
              21,
              22
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
        "reference": 15,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "r1",
        "typing": {
          "status": "absent"
        },
        "owner": 19,
        "source": 7,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 16,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "s2",
        "typing": {
          "status": "absent"
        },
        "owner": 19,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 17,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "s3",
        "typing": {
          "status": "absent"
        },
        "owner": 19,
        "source": 9,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 18,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "r",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 5,
              "label": "R"
            }
          ]
        },
        "owner": 0,
        "source": 10,
        "compartments": [
          {
            "kind": "occurrences",
            "provenance": "direct",
            "members": [
              24
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
        "reference": 19,
        "metaclass": "OccurrenceUsage",
        "notationRole": "usage",
        "name": "r2",
        "typing": {
          "status": "absent"
        },
        "owner": 23,
        "source": 12,
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
        "reference": 50,
        "source": 16,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 3
        },
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 63,
        "source": 19,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 4
        },
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 69,
        "source": 23,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 5
        },
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 43,
        "source": 6,
        "kind": "flowPayloadType",
        "target": {
          "status": "resolved",
          "reference": 2
        },
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 44,
        "source": 6,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 22
        },
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 45,
        "source": 6,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 55,
        "source": 11,
        "kind": "flowPayloadType",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 18
      },
      {
        "reference": 56,
        "source": 11,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 18
        },
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 57,
        "source": 11,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 20
        },
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 72,
        "source": 1,
        "kind": "flowPayloadType",
        "target": {
          "status": "resolved",
          "reference": 1
        },
        "provenance": "authored",
        "navigation": 22
      },
      {
        "reference": 73,
        "source": 1,
        "kind": "flowSource",
        "target": {
          "status": "resolved",
          "node": 21
        },
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 74,
        "source": 1,
        "kind": "flowTarget",
        "target": {
          "status": "resolved",
          "node": 24
        },
        "provenance": "authored",
        "navigation": 24
      }
    ],
    "edges": [
      {
        "reference": 35,
        "source": 0,
        "target": 16,
        "origin": 16,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 48,
        "source": 16,
        "target": 17,
        "origin": 17,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 52,
        "source": 18,
        "target": 17,
        "origin": 17,
        "kind": "succession",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 49,
        "source": 16,
        "target": 18,
        "origin": 18,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 36,
        "source": 0,
        "target": 19,
        "origin": 19,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 60,
        "source": 19,
        "target": 20,
        "origin": 20,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 61,
        "source": 19,
        "target": 21,
        "origin": 21,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 64,
        "source": 20,
        "target": 21,
        "origin": 21,
        "kind": "succession",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 62,
        "source": 19,
        "target": 22,
        "origin": 22,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 66,
        "source": 21,
        "target": 22,
        "origin": 22,
        "kind": "succession",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 37,
        "source": 0,
        "target": 23,
        "origin": 23,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 68,
        "source": 23,
        "target": 24,
        "origin": 24,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 38,
        "source": 0,
        "target": 6,
        "origin": 6,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 67,
        "source": 22,
        "target": 17,
        "origin": 6,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 14
      },
      {
        "reference": 41,
        "source": 6,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 46,
        "source": 7,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 42,
        "source": 6,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 47,
        "source": 9,
        "target": 10,
        "origin": 10,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 39,
        "source": 0,
        "target": 11,
        "origin": 11,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 17
      },
      {
        "reference": 51,
        "source": 18,
        "target": 20,
        "origin": 11,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 18
      },
      {
        "reference": 53,
        "source": 11,
        "target": 12,
        "origin": 12,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 58,
        "source": 12,
        "target": 13,
        "origin": 13,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 54,
        "source": 11,
        "target": 14,
        "origin": 14,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 59,
        "source": 14,
        "target": 15,
        "origin": 15,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 40,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 21
      },
      {
        "reference": 65,
        "source": 21,
        "target": 24,
        "origin": 1,
        "kind": "flow",
        "provenance": "implied",
        "navigation": 22
      },
      {
        "reference": 70,
        "source": 1,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 75,
        "source": 2,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 71,
        "source": 1,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 24
      },
      {
        "reference": 76,
        "source": 4,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 24
      }
    ],
    "metadata": {
      "participants": [
        16,
        19,
        23
      ],
      "messages": [
        1,
        6,
        11
      ]
    },
    "scene": {
      "kind": "sequence",
      "lifelines": [
        16,
        19,
        23
      ],
      "messages": [
        {
          "node": 1,
          "label": "telemetryMsg",
          "source": {
            "status": "resolved",
            "lifeline": 19
          },
          "target": {
            "status": "resolved",
            "lifeline": 23
          },
          "order": {
            "status": "resolved",
            "value": 2
          },
          "provenance": "authored",
          "navigation": 21
        },
        {
          "node": 6,
          "label": "ackMsg",
          "source": {
            "status": "resolved",
            "lifeline": 19
          },
          "target": {
            "status": "resolved",
            "lifeline": 16
          },
          "order": {
            "status": "resolved",
            "value": 3
          },
          "provenance": "authored",
          "navigation": 13
        },
        {
          "node": 11,
          "label": "pressMsg",
          "source": {
            "status": "resolved",
            "lifeline": 16
          },
          "target": {
            "status": "resolved",
            "lifeline": 19
          },
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "navigation": 17
        }
      ]
    }
  }
}

~~~
