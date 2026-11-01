# META
~~~ini
description=Issue 286: cyclic event precedence stays explicitly unordered
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_event_order_event_cycle.md
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
        succession first q.s2 then q.r1;
    }
    view seq : StandardViewDefinitions::SequenceView { expose Scenario; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:bf44151f51f358d0491ef87eaf7592bbd6bd9c12520bf3c30b84620d718d6dc3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::A"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::B"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::C"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "q::s2")) (succession (reference "q::r1")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "q::s3")) (flowTarget (reference "p::r3")) (flowPayloadType (reference "C")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "p::s1")) (flowTarget (reference "q::r1")) (flowPayloadType (reference "A")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r::r2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "q::s2")) (flowTarget (reference "r::r2")) (flowPayloadType (reference "B")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Scenario")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowSource) (ordinal 0))
      (authored-target "q::s3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowTarget) (ordinal 0))
      (authored-target "p::r3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::C")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowSource) (ordinal 0))
      (authored-target "p::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowTarget) (ordinal 0))
      (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::A")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowSource) (ordinal 0))
      (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowTarget) (ordinal 0))
      (authored-target "r::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r::r2")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::B")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r::r2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::A"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::B"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::C"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r::r2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r::r2"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::A")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::B")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::C")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "ackMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "pressMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r::r2")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "telemetryMsg")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 10 25) (end 10 29)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 10 35) (end 10 39)) (probe (position 10 35))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 7 33) (end 7 37)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowSource) (ordinal 0) (authored-target "q::s3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s3")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 7 41) (end 7 45)) (probe (position 7 41))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowTarget) (ordinal 0) (authored-target "p::r3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::r3")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 7 26) (end 7 27)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::ackMsg"))) (kind flowPayloadType) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::C")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 4 17) (end 4 18)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::P")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 8 35) (end 8 39)) (probe (position 8 35))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowSource) (ordinal 0) (authored-target "p::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::p::s1")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 8 43) (end 8 47)) (probe (position 8 43))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowTarget) (ordinal 0) (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::r1")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 8 28) (end 8 29)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::pressMsg"))) (kind flowPayloadType) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::A")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 5 17) (end 5 18)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Q")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 6 17) (end 6 18)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::R")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 9 39) (end 9 43)) (probe (position 9 39))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowSource) (ordinal 0) (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::q::s2")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 9 47) (end 9 51)) (probe (position 9 47))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowTarget) (ordinal 0) (authored-target "r::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::r::r2")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 9 32) (end 9 33)) (probe (position 9 32))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario::telemetryMsg"))) (kind flowPayloadType) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::B")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 12 15) (end 12 52)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (range (start 12 62) (end 12 70)) (probe (position 12 62))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_event_cycle.md") (qualified-name "Seq::Scenario")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:c497b00a58fd6920dee562a793d8157ddc766ba7615b92747ead4c3685828f65",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_event_order_event_cycle.md",
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
        10,
        8,
        10,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        25,
        10,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        35,
        10,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        9,
        12,
        12
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::A"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::B"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::C"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::P"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Q"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::R"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::ackMsg"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::ackMsg::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::p"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::p::r3"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::p::s1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::pressMsg"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::pressMsg::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::q"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::q::r1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::q::s2"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::q::s3"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::r"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::r::r2"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::telemetryMsg"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::telemetryMsg::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::seq"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Seq::Scenario",
      "source": 25,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::ackMsg",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::ackMsg",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::ackMsg::",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::ackMsg::",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::pressMsg",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::pressMsg",
      "source": 20,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::pressMsg::",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::pressMsg::",
      "source": 20,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg",
      "source": 23,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg::",
      "source": 23,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::telemetryMsg::",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "flowPayloadType",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "flowSource",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "flowTarget",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "flow",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "flowPayloadType",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "flowSource",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "flowTarget",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "succession",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "flow",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "succession",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "flow",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "typing",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "flowPayloadType",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "flowSource",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "flowTarget",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "containment",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "containment",
      "source": 22
    }
  ],
  "selectedView": {
    "reference": 23,
    "kind": "sequence-view",
    "name": "seq",
    "source": 28
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "sequence-ordering-cycle",
        "witness": [
          {
            "events": [
              22,
              21,
              22
            ],
            "provenance": "implied",
            "source": 2,
            "target": 2
          }
        ]
      },
      {
        "code": "sequence-ordering-cycle",
        "witness": [
          {
            "events": [
              21,
              22,
              21
            ],
            "provenance": "implied",
            "source": 12,
            "target": 12
          }
        ]
      }
    ]
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 17,
        "provenance": "authored",
        "reference": 37,
        "source": 0,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 18,
        "provenance": "authored",
        "reference": 53,
        "source": 17,
        "target": 18
      },
      {
        "kind": "succession",
        "navigation": 4,
        "origin": 18,
        "provenance": "authored",
        "reference": 57,
        "source": 19,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 19,
        "provenance": "authored",
        "reference": 54,
        "source": 17,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 20,
        "provenance": "authored",
        "reference": 38,
        "source": 0,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 21,
        "provenance": "authored",
        "reference": 65,
        "source": 20,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 22,
        "provenance": "authored",
        "reference": 66,
        "source": 20,
        "target": 22
      },
      {
        "kind": "succession",
        "navigation": 8,
        "origin": 22,
        "provenance": "authored",
        "reference": 69,
        "source": 21,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 23,
        "provenance": "authored",
        "reference": 67,
        "source": 20,
        "target": 23
      },
      {
        "kind": "succession",
        "navigation": 9,
        "origin": 23,
        "provenance": "authored",
        "reference": 71,
        "source": 22,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 24,
        "provenance": "authored",
        "reference": 39,
        "source": 0,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 25,
        "provenance": "authored",
        "reference": 74,
        "source": 24,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 1,
        "provenance": "authored",
        "reference": 40,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 26,
        "origin": 1,
        "provenance": "implied",
        "reference": 72,
        "source": 22,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 7,
        "provenance": "authored",
        "reference": 41,
        "source": 0,
        "target": 7
      },
      {
        "kind": "flow",
        "navigation": 14,
        "origin": 7,
        "provenance": "implied",
        "reference": 73,
        "source": 23,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 8,
        "provenance": "authored",
        "reference": 46,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 9,
        "provenance": "authored",
        "reference": 51,
        "source": 8,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 10,
        "provenance": "authored",
        "reference": 47,
        "source": 7,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 11,
        "provenance": "authored",
        "reference": 52,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 12,
        "provenance": "authored",
        "reference": 42,
        "source": 0,
        "target": 12
      },
      {
        "kind": "flow",
        "navigation": 18,
        "origin": 12,
        "provenance": "implied",
        "reference": 56,
        "source": 19,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 13,
        "provenance": "authored",
        "reference": 58,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 14,
        "provenance": "authored",
        "reference": 63,
        "source": 13,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 15,
        "provenance": "authored",
        "reference": 59,
        "source": 12,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 16,
        "provenance": "authored",
        "reference": 64,
        "source": 15,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 21,
        "origin": 2,
        "provenance": "authored",
        "reference": 43,
        "source": 0,
        "target": 2
      },
      {
        "kind": "flow",
        "navigation": 22,
        "origin": 2,
        "provenance": "implied",
        "reference": 70,
        "source": 22,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 3,
        "provenance": "authored",
        "reference": 76,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 4,
        "provenance": "authored",
        "reference": 81,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 5,
        "provenance": "authored",
        "reference": 77,
        "source": 2,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 6,
        "provenance": "authored",
        "reference": 82,
        "source": 5,
        "target": 6
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        2,
        7,
        12
      ],
      "participants": [
        17,
        20,
        24
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              17,
              20,
              24
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              2,
              7,
              12
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "OccurrenceDefinition",
        "multiplicity": null,
        "name": "Scenario",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:bd89a0e8a9482d207fdb6eeab2512d61e012226cd9b2984c6ab1c5f97b91f521",
        "reference": 6,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:2bd50e1e9cf9e28acabc8d329c102e6c0032a788f729b30eda22dfe11b1dc331",
        "reference": 24,
        "source": 25,
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
        "name": "telemetryMsg",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:88ba6dd301189fbdee50464d0f0b60bb19800da277fdab91f062be78a8dc833a",
        "reference": 21,
        "source": 21,
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
        "owner": 2,
        "presentationKey": "p:06b4be6c280ea54a0336569f17c09947dc2263a660166c37261c8c549522fa8d",
        "reference": 33,
        "source": 23,
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
        "owner": 3,
        "presentationKey": "p:7ee67e0b3b1c52cd2fe1ebfe97ce70c1ac9a9ddead46fb054a4d388ced50026f",
        "reference": 35,
        "source": 23,
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
        "owner": 2,
        "presentationKey": "p:dec796974bb6a0c74730ce1799574ebd704656a0fac6fc724d86ce4c51b25e69",
        "reference": 34,
        "source": 24,
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
        "owner": 5,
        "presentationKey": "p:b3d9893f2a8a297e5ec05d6f187c6e21847e38fd54b09f669162988e6eccac19",
        "reference": 36,
        "source": 24,
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
        "name": "ackMsg",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:8314271aceea9c2d7568deb13f8e1d023befefec2c560b59df5a9a3e1ae42f29",
        "reference": 8,
        "source": 13,
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
        "owner": 7,
        "presentationKey": "p:fda665bb5dcf757d289d6c30f80e2839afb996c176ed4ac55c7b7bbb110de506",
        "reference": 25,
        "source": 15,
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
        "owner": 8,
        "presentationKey": "p:53c30f3463ee7129675c3b3badde9fa0749284abb4039774756cc566e6ecb81a",
        "reference": 27,
        "source": 15,
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
        "owner": 7,
        "presentationKey": "p:e38f905e91ed9cecfe7f5a778ff4e2aa84a6e069d966105d926dd5d870b9b66b",
        "reference": 26,
        "source": 16,
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
        "owner": 10,
        "presentationKey": "p:2a8a15c82ee0d2674074534cd6423983fdb028827ea474e1270ab9b0bc95dceb",
        "reference": 28,
        "source": 16,
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
        "name": "pressMsg",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d23c2d51f4e88a9134e376c21c691308cb25d780588dc025dc8affd2dd2e99bd",
        "reference": 13,
        "source": 17,
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
        "owner": 12,
        "presentationKey": "p:7d0058319dcce299444347caff7b56d2e53808da6a3796b7e6f5705bd85c45c4",
        "reference": 29,
        "source": 19,
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
        "owner": 13,
        "presentationKey": "p:f7b6836d6b9efd237371985d7f053c606095e87bdaa9168c791b0e2227af4ec0",
        "reference": 31,
        "source": 19,
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
        "owner": 12,
        "presentationKey": "p:6d9a167c970502fca14d91018654498cceafa6face77b956d4cb76e0bffcd375",
        "reference": 30,
        "source": 20,
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
        "owner": 15,
        "presentationKey": "p:a68aa004a66de7ed35d2a6ed34adc10d192ced7bdf5ba1d0d09e3839d99531f3",
        "reference": 32,
        "source": 20,
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
              18,
              19
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "p",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:48bbfe44cd9001b42a0423f2af7356174a8e42a8b00d437cfb5bb0b38214f7ef",
        "reference": 10,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 3
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
        "multiplicity": null,
        "name": "r3",
        "notationRole": "usage",
        "owner": 17,
        "presentationKey": "p:7f8397881a228e59ed8b5472d57708f39588d5ad6766761839bbb7ba45c82c88",
        "reference": 11,
        "source": 4,
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
        "name": "s1",
        "notationRole": "usage",
        "owner": 17,
        "presentationKey": "p:15ca8876178d30574c062dfb7265909bc18e5e7d4fae667c0861e1b141e3d6cd",
        "reference": 12,
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
              21,
              22,
              23
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "q",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:0ee158d53b160a7ca1af8da9c86c0731dfaa32d0ed973ef92d43154af1208924",
        "reference": 15,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Q",
              "reference": 4
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
        "multiplicity": null,
        "name": "r1",
        "notationRole": "usage",
        "owner": 20,
        "presentationKey": "p:731824dbc7f4258056b0e719e12d77272852c7550045eae76955d180e08d74e1",
        "reference": 16,
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
        "metaclass": "OccurrenceUsage",
        "multiplicity": null,
        "name": "s2",
        "notationRole": "usage",
        "owner": 20,
        "presentationKey": "p:0e271bd1a054f097abdad59f7135d40e40ec719c842c1ba11f441852c04b1e4d",
        "reference": 17,
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
        "metaclass": "OccurrenceUsage",
        "multiplicity": null,
        "name": "s3",
        "notationRole": "usage",
        "owner": 20,
        "presentationKey": "p:77e77647e69a08d3773022d5233463767d627e466a9d52a15c6ba5b5e1a5e17a",
        "reference": 18,
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
              25
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "r",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:f7eb7048882c5f4211f49bf0784d426f867fb9915026e0eb600bb85612bb5406",
        "reference": 19,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "R",
              "reference": 5
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
        "multiplicity": null,
        "name": "r2",
        "notationRole": "usage",
        "owner": 24,
        "presentationKey": "p:defac9c141933e8af9d43dc03c17437b668fa9b483ed87b51725ba34133d4827",
        "reference": 20,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 55,
        "source": 17,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 68,
        "source": 20,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 75,
        "source": 24,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 26,
        "provenance": "authored",
        "reference": 44,
        "source": 1,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 27,
        "provenance": "authored",
        "reference": 45,
        "source": 1,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "flowPayloadType",
        "navigation": 14,
        "provenance": "authored",
        "reference": 48,
        "source": 7,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 15,
        "provenance": "authored",
        "reference": 49,
        "source": 7,
        "target": {
          "node": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 16,
        "provenance": "authored",
        "reference": 50,
        "source": 7,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "flowPayloadType",
        "navigation": 18,
        "provenance": "authored",
        "reference": 60,
        "source": 12,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 19,
        "provenance": "authored",
        "reference": 61,
        "source": 12,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 20,
        "provenance": "authored",
        "reference": 62,
        "source": 12,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "flowPayloadType",
        "navigation": 22,
        "provenance": "authored",
        "reference": 78,
        "source": 2,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 23,
        "provenance": "authored",
        "reference": 79,
        "source": 2,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 24,
        "provenance": "authored",
        "reference": 80,
        "source": 2,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        17,
        20,
        24
      ],
      "messages": [
        {
          "label": "telemetryMsg",
          "navigation": 21,
          "node": 2,
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "source": {
            "lifeline": 20,
            "status": "resolved"
          },
          "target": {
            "lifeline": 24,
            "status": "resolved"
          }
        },
        {
          "label": "ackMsg",
          "navigation": 13,
          "node": 7,
          "order": {
            "status": "blocked-by-cycle"
          },
          "provenance": "authored",
          "source": {
            "lifeline": 20,
            "status": "resolved"
          },
          "target": {
            "lifeline": 17,
            "status": "resolved"
          }
        },
        {
          "label": "pressMsg",
          "navigation": 17,
          "node": 12,
          "order": {
            "status": "cyclic"
          },
          "provenance": "authored",
          "source": {
            "lifeline": 17,
            "status": "resolved"
          },
          "target": {
            "lifeline": 20,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
