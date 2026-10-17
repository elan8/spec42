# META
~~~ini
description=Issue 286: message order follows event successions across lifelines despite names and declaration order
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=diagram_sequence_event_order_reversed_names.md
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
        message c of A from p.s1 to q.r1;
        message b of B from q.s2 to r.r2;
        message a of C from q.s3 to p.r3;
    }
    view seq : StandardViewDefinitions::SequenceView { expose Scenario; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:019a8e863f9943244825049b34f65959a2a85540d489a99822947da326c6d21a") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::A"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::B"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::C"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "q::s3")) (flowTarget (reference "p::r3")) (flowPayloadType (reference "C")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "q::s2")) (flowTarget (reference "r::r2")) (flowPayloadType (reference "B")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "p::s1")) (flowTarget (reference "q::r1")) (flowPayloadType (reference "A")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::r3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::s1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::r1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "R")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r::r2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::SequenceView")))))
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Scenario")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowSource) (ordinal 0))
      (authored-target "q::s3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s3")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowTarget) (ordinal 0))
      (authored-target "p::r3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::r3")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::C")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowSource) (ordinal 0))
      (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s2")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowTarget) (ordinal 0))
      (authored-target "r::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r::r2")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::B")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowSource) (ordinal 0))
      (authored-target "p::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::s1")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowTarget) (ordinal 0))
      (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::r1")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::A")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::r3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r::r2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::s1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::r1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::A"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::B"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::C"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::r3"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::r3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::s1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::s1"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::r1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::r1"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s2"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s3"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s3"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r::r2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r::r2"))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::A")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::B")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::C")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "a")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "b")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind occurrence-def) (name "Scenario")) (named (kind flow) (name "c")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::r3")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::s1")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::r1")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s2")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s3")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))
      (type (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r::r2")))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq")))
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
    (declaration (id (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 9 28) (end 9 32)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowSource) (ordinal 0) (authored-target "q::s3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s3")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 9 36) (end 9 40)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowTarget) (ordinal 0) (authored-target "p::r3")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::r3")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 9 21) (end 9 22)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::a"))) (kind flowPayloadType) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::C")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 8 28) (end 8 32)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowSource) (ordinal 0) (authored-target "q::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::s2")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 8 36) (end 8 40)) (probe (position 8 36))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowTarget) (ordinal 0) (authored-target "r::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r::r2")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 8 21) (end 8 22)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::b"))) (kind flowPayloadType) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::B")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 7 28) (end 7 32)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowSource) (ordinal 0) (authored-target "p::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p::s1")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 7 36) (end 7 40)) (probe (position 7 36))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowTarget) (ordinal 0) (authored-target "q::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q::r1")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 7 21) (end 7 22)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::c"))) (kind flowPayloadType) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::A")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 4 17) (end 4 18)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::p"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::P")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 5 17) (end 5 18)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::q"))) (kind featureTyping) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Q")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 6 17) (end 6 18)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario::r"))) (kind featureTyping) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::R")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 11 15) (end 11 52)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::seq"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (range (start 11 62) (end 11 70)) (probe (position 11 62))
    (reference (id (source (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (path (named (kind package) (name "Seq")) (named (kind view) (name "seq")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_sequence_event_order_reversed_names.md") (qualified-name "Seq::Scenario")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:2758d522fdb6abcfa65905e3f927c7f23ccff8e94971f345d5b3505d508746bf",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_sequence_event_order_reversed_names.md",
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
        17
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        21,
        7,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        28,
        7,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        36,
        7,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        16,
        8,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        21,
        8,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        28,
        8,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        36,
        8,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        16,
        9,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        21,
        9,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        28,
        9,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        36,
        9,
        40
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
      "qualifiedName": "Seq::Scenario::a"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::a::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::a::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::b"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::b::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::b::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::c"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::c::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Seq::Scenario::c::::"
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
      "qualifiedName": "Seq::seq"
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
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::a",
      "source": 23,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::a",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::a::",
      "source": 23,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::a::",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::b",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::b",
      "source": 20,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::b::",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::b::",
      "source": 20,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::c",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Seq::Scenario::c",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::c::",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Seq::Scenario::c::",
      "source": 16,
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
      "ordinal": 6,
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
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "flowPayloadType",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "flowSource",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "flowTarget",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "subsetting",
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
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 9,
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
      "ordinal": 5,
      "relationshipKind": "containment",
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
      "ordinal": 14,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "redefinition",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "redefinition",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typeFeaturing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typeFeaturing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "flowPayloadType",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "flowSource",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "flowTarget",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "typeFeaturing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "redefinition",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "redefinition",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "flowPayloadType",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "flowSource",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "flowTarget",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "typeFeaturing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "redefinition",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "redefinition",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "subsetting",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "subsetting",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "typeFeaturing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "typeFeaturing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "subsetting",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "subsetting",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "typeFeaturing",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "typing",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "subsetting",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "typeFeaturing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "flow",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "subsetting",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "succession",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "typeFeaturing",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "subsetting",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "subsetting",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "typeFeaturing",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "typing",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 64,
      "relationshipKind": "subsetting",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "succession",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "typeFeaturing",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "flow",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "subsetting",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "succession",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "typeFeaturing",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "flow",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "subsetting",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "typeFeaturing",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "containment",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 71,
      "relationshipKind": "subsetting",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 72,
      "relationshipKind": "subsetting",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 73,
      "relationshipKind": "typeFeaturing",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "typing",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 74,
      "relationshipKind": "subsetting",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "typeFeaturing",
      "source": 24
    }
  ],
  "selectedView": {
    "reference": 25,
    "kind": "sequence-view",
    "name": "seq",
    "source": 25
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 21,
        "origin": 1,
        "provenance": "authored",
        "reference": 49,
        "source": 0,
        "target": 1
      },
      {
        "kind": "flow",
        "navigation": 22,
        "origin": 1,
        "provenance": "implied",
        "reference": 145,
        "source": 22,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 2,
        "provenance": "authored",
        "reference": 56,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 3,
        "provenance": "authored",
        "reference": 67,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 4,
        "provenance": "authored",
        "reference": 57,
        "source": 1,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 5,
        "provenance": "authored",
        "reference": 68,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 6,
        "provenance": "authored",
        "reference": 50,
        "source": 0,
        "target": 6
      },
      {
        "kind": "flow",
        "navigation": 18,
        "origin": 6,
        "provenance": "implied",
        "reference": 141,
        "source": 21,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 7,
        "provenance": "authored",
        "reference": 77,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 8,
        "provenance": "authored",
        "reference": 88,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 9,
        "provenance": "authored",
        "reference": 78,
        "source": 6,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 10,
        "provenance": "authored",
        "reference": 89,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 11,
        "provenance": "authored",
        "reference": 51,
        "source": 0,
        "target": 11
      },
      {
        "kind": "flow",
        "navigation": 14,
        "origin": 11,
        "provenance": "implied",
        "reference": 127,
        "source": 18,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 12,
        "provenance": "authored",
        "reference": 98,
        "source": 11,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 13,
        "provenance": "authored",
        "reference": 109,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 14,
        "provenance": "authored",
        "reference": 99,
        "source": 11,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 15,
        "provenance": "authored",
        "reference": 110,
        "source": 14,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 16,
        "provenance": "authored",
        "reference": 52,
        "source": 0,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 17,
        "provenance": "authored",
        "reference": 119,
        "source": 16,
        "target": 17
      },
      {
        "kind": "succession",
        "navigation": 4,
        "origin": 17,
        "provenance": "authored",
        "reference": 129,
        "source": 18,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 18,
        "provenance": "authored",
        "reference": 120,
        "source": 16,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 19,
        "provenance": "authored",
        "reference": 53,
        "source": 0,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 20,
        "provenance": "authored",
        "reference": 131,
        "source": 19,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 21,
        "provenance": "authored",
        "reference": 132,
        "source": 19,
        "target": 21
      },
      {
        "kind": "succession",
        "navigation": 8,
        "origin": 21,
        "provenance": "authored",
        "reference": 139,
        "source": 20,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 22,
        "provenance": "authored",
        "reference": 133,
        "source": 19,
        "target": 22
      },
      {
        "kind": "succession",
        "navigation": 9,
        "origin": 22,
        "provenance": "authored",
        "reference": 143,
        "source": 21,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 23,
        "provenance": "authored",
        "reference": 54,
        "source": 0,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 24,
        "provenance": "authored",
        "reference": 148,
        "source": 23,
        "target": 24
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        1,
        6,
        11
      ],
      "participants": [
        16,
        19,
        23
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              16,
              19,
              23
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1,
              6,
              11
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
        "metaclass": "FlowUsage",
        "name": "a",
        "notationRole": "usage",
        "owner": 0,
        "reference": 7,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "reference": 37,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "reference": 39,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "reference": 38,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 4,
        "reference": 40,
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
        "name": "b",
        "notationRole": "usage",
        "owner": 0,
        "reference": 10,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 6,
        "reference": 41,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 7,
        "reference": 43,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 6,
        "reference": 42,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 9,
        "reference": 44,
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
        "metaclass": "FlowUsage",
        "name": "c",
        "notationRole": "usage",
        "owner": 0,
        "reference": 13,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 11,
        "reference": 45,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 12,
        "reference": 47,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 11,
        "reference": 46,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 14,
        "reference": 48,
        "source": 16,
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
              17,
              18
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
        "reference": 16,
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
        "name": "r3",
        "notationRole": "usage",
        "owner": 16,
        "reference": 17,
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
        "name": "s1",
        "notationRole": "usage",
        "owner": 16,
        "reference": 18,
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
              20,
              21,
              22
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
        "reference": 19,
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
        "name": "r1",
        "notationRole": "usage",
        "owner": 19,
        "reference": 20,
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
        "name": "s2",
        "notationRole": "usage",
        "owner": 19,
        "reference": 21,
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
        "name": "s3",
        "notationRole": "usage",
        "owner": 19,
        "reference": 22,
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
              24
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "r",
        "notationRole": "usage",
        "owner": 0,
        "reference": 23,
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
        "name": "r2",
        "notationRole": "usage",
        "owner": 23,
        "reference": 24,
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
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 55,
        "source": 0,
        "target": {
          "reference": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "flowPayloadType",
        "navigation": 22,
        "provenance": "authored",
        "reference": 58,
        "source": 1,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 23,
        "provenance": "authored",
        "reference": 59,
        "source": 1,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 24,
        "provenance": "authored",
        "reference": 60,
        "source": 1,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 61,
        "source": 1,
        "target": {
          "reference": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 1,
        "target": {
          "reference": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 63,
        "source": 1,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 64,
        "source": 1,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 65,
        "source": 1,
        "target": {
          "reference": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 66,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 69,
        "source": 2,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 71,
        "source": 3,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 73,
        "source": 3,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 75,
        "source": 3,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 70,
        "source": 4,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 72,
        "source": 5,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 74,
        "source": 5,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 76,
        "source": 5,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "flowPayloadType",
        "navigation": 18,
        "provenance": "authored",
        "reference": 79,
        "source": 6,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 19,
        "provenance": "authored",
        "reference": 80,
        "source": 6,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 20,
        "provenance": "authored",
        "reference": 81,
        "source": 6,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 82,
        "source": 6,
        "target": {
          "reference": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 83,
        "source": 6,
        "target": {
          "reference": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 84,
        "source": 6,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 85,
        "source": 6,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 86,
        "source": 6,
        "target": {
          "reference": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 87,
        "source": 6,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 90,
        "source": 7,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 92,
        "source": 8,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 94,
        "source": 8,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 96,
        "source": 8,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 91,
        "source": 9,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 93,
        "source": 10,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 95,
        "source": 10,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 97,
        "source": 10,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "flowPayloadType",
        "navigation": 14,
        "provenance": "authored",
        "reference": 100,
        "source": 11,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 15,
        "provenance": "authored",
        "reference": 101,
        "source": 11,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 16,
        "provenance": "authored",
        "reference": 102,
        "source": 11,
        "target": {
          "node": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 103,
        "source": 11,
        "target": {
          "reference": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 104,
        "source": 11,
        "target": {
          "reference": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 105,
        "source": 11,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 106,
        "source": 11,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 107,
        "source": 11,
        "target": {
          "reference": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 108,
        "source": 11,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 111,
        "source": 12,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 113,
        "source": 13,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 115,
        "source": 13,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 117,
        "source": 13,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 112,
        "source": 14,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 114,
        "source": 15,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 116,
        "source": 15,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 118,
        "source": 15,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 124,
        "source": 16,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 121,
        "source": 16,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 122,
        "source": 16,
        "target": {
          "reference": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 123,
        "source": 16,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 125,
        "source": 17,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 126,
        "source": 17,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 128,
        "source": 18,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 130,
        "source": 18,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 137,
        "source": 19,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 134,
        "source": 19,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 135,
        "source": 19,
        "target": {
          "reference": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 136,
        "source": 19,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 138,
        "source": 20,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 140,
        "source": 20,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 142,
        "source": 21,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 144,
        "source": 21,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 146,
        "source": 22,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 147,
        "source": 22,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 152,
        "source": 23,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 149,
        "source": 23,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 150,
        "source": 23,
        "target": {
          "reference": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 151,
        "source": 23,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 153,
        "source": 24,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 154,
        "source": 24,
        "target": {
          "node": 23,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        16,
        19,
        23
      ],
      "messages": [
        {
          "label": "a",
          "navigation": 21,
          "node": 1,
          "order": {
            "status": "resolved",
            "value": 3
          },
          "provenance": "authored",
          "source": {
            "lifeline": 19,
            "status": "resolved"
          },
          "target": {
            "lifeline": 16,
            "status": "resolved"
          }
        },
        {
          "label": "b",
          "navigation": 17,
          "node": 6,
          "order": {
            "status": "resolved",
            "value": 2
          },
          "provenance": "authored",
          "source": {
            "lifeline": 19,
            "status": "resolved"
          },
          "target": {
            "lifeline": 23,
            "status": "resolved"
          }
        },
        {
          "label": "c",
          "navigation": 13,
          "node": 11,
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "source": {
            "lifeline": 16,
            "status": "resolved"
          },
          "target": {
            "lifeline": 19,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
