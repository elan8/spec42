# META
~~~ini
description=Sequence ordering: an authored order of receive events on one lifeline orders the messages received there; SysML 2.0 8.2.3.9
type=generate
libraries=standard
plugin=native:diagram
viewKind=sequence-view
viewDocument=model.sysml
viewQualifiedName=Repro::selected
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Repro {
    private import StandardViewDefinitions::*;
    occurrence def Interaction {
        part A { event occurrence s1; }
        part C { event occurrence s2; }
        part B {
            event occurrence r1;
            event occurrence r2;
            first r1 then r2;
        }
        message M2 from C.s2 to B.r2;
        message M1 from A.s1 to B.r1;
    }
    view selected : SequenceView { expose Interaction; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:fc61f7620fbb3faaf829fd27a19df3c0ffc426bcd4e3b2f50189e9af85ad941a") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A::s1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "r1")) (succession (reference "r2")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C::s2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "A::s1")) (flowTarget (reference "B::r1")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "C::s2")) (flowTarget (reference "B::r2")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SequenceView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Interaction")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (kind flowSource) (ordinal 0))
      (authored-target "A::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A::s1")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (kind flowTarget) (ordinal 0))
      (authored-target "B::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (kind flowSource) (ordinal 0))
      (authored-target "C::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C::s2")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (kind flowTarget) (ordinal 0))
      (authored-target "B::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A::s1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C::s2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A::s1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A::s1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C::s2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C::s2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A::s1")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C::s2")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M1")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind flow) (name "M2")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 18) (end 8 20)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 26) (end 8 28)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind occurrence-def) (name "Interaction")) (named (kind part) (name "B")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 24) (end 11 28)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (kind flowSource) (ordinal 0) (authored-target "A::s1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::A::s1")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 32) (end 11 36)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M1"))) (kind flowTarget) (ordinal 0) (authored-target "B::r1")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r1")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 24) (end 10 28)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (kind flowSource) (ordinal 0) (authored-target "C::s2")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::C::s2")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 32) (end 10 36)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::M2"))) (kind flowTarget) (ordinal 0) (authored-target "B::r2")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction::B::r2")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 20) (end 13 32)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::selected"))) (kind featureTyping) (ordinal 0) (authored-target "SequenceView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 42) (end 13 53)) (probe (position 13 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Repro")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Interaction")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Repro::Interaction")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:27739285f226b99f4cd94f747c7472e2082d14cd0ff07da5645a26e69306b0af",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
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
        30
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
        34,
        3,
        36
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
        34,
        4,
        36
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
        6,
        29,
        6,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        29,
        7,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        12,
        8,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        18,
        8,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        26,
        8,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        16,
        10,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        24,
        10,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        32,
        10,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        16,
        11,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        24,
        11,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        32,
        11,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        9,
        13,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::A"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::A::s1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::B"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::B::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::B::r1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::B::r2"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::C"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::C::s2"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::M1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::M1::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::M2"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::Interaction::M2::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Repro::selected"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Repro::Interaction::B",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::M1",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::M1",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::M1::",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::M1::",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::M2",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Repro::Interaction::M2",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::M2::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Repro::Interaction::M2::",
      "source": 13,
      "sourceDomain": "workspace"
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
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "flow",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "succession",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "flow",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "flowSource",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "flowTarget",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "flowSource",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "flowTarget",
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
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 12
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
    "edges": [
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 11,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 12,
        "provenance": "authored",
        "reference": 28,
        "source": 11,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 13,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 16,
        "provenance": "authored",
        "reference": 30,
        "source": 13,
        "target": 16
      },
      {
        "kind": "succession",
        "navigation": 9,
        "origin": 16,
        "provenance": "implied",
        "reference": 35,
        "source": 14,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 14,
        "provenance": "authored",
        "reference": 31,
        "source": 13,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 15,
        "provenance": "authored",
        "reference": 32,
        "source": 13,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 17,
        "provenance": "authored",
        "reference": 25,
        "source": 0,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 18,
        "provenance": "authored",
        "reference": 36,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 1,
        "provenance": "authored",
        "reference": 26,
        "source": 0,
        "target": 1
      },
      {
        "kind": "flow",
        "navigation": 15,
        "origin": 1,
        "provenance": "implied",
        "reference": 29,
        "source": 12,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 2,
        "provenance": "authored",
        "reference": 38,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 3,
        "provenance": "authored",
        "reference": 42,
        "source": 2,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 4,
        "provenance": "authored",
        "reference": 39,
        "source": 1,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 5,
        "provenance": "authored",
        "reference": 43,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 6,
        "provenance": "authored",
        "reference": 27,
        "source": 0,
        "target": 6
      },
      {
        "kind": "flow",
        "navigation": 12,
        "origin": 6,
        "provenance": "implied",
        "reference": 37,
        "source": 18,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 7,
        "provenance": "authored",
        "reference": 44,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 8,
        "provenance": "authored",
        "reference": 48,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 9,
        "provenance": "authored",
        "reference": 45,
        "source": 6,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 10,
        "provenance": "authored",
        "reference": 49,
        "source": 9,
        "target": 10
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "sequence-view",
    "metadata": {
      "messages": [
        1,
        6
      ],
      "participants": [
        11,
        17,
        13
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              11,
              13,
              17
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1,
              6
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "OccurrenceDefinition",
        "multiplicity": null,
        "name": "Interaction",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:90929811bb5f24ee043cfb236a5a3e73bb0f477706fded0660ebabf90c21697a",
        "reference": 0,
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
        "multiplicity": null,
        "name": "M1",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:33cd7e4a4d7066099ac5202a06071ec47f750129b59718c16cecbf614feb30b1",
        "reference": 9,
        "source": 14,
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
        "owner": 1,
        "presentationKey": "p:a0043a8411f7d58d766adadbaee16b7668797033afd54b8a9d30b3433581f567",
        "reference": 15,
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
        "owner": 2,
        "presentationKey": "p:4e513dc752270962436a48ef132c1c015f68b40e8c827e87d965957b31bf4283",
        "reference": 17,
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
        "owner": 1,
        "presentationKey": "p:eb3506286333e6c35c910fbbe0692567896f605af233825f4543dc345af78aa3",
        "reference": 16,
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
        "owner": 4,
        "presentationKey": "p:1539d92108cbba88860a21658251d073c95ad70b6696515a41566bc67fbcbb95",
        "reference": 18,
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
        "name": "M2",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:1db79142f5872e5500ba422c23a3283a232aae2df05945a55aed02ce531e7f19",
        "reference": 11,
        "source": 11,
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
        "owner": 6,
        "presentationKey": "p:6c18571a210d155ef32be523ac6949cf4177af428e8463e3c2fd7c8b5ca8664d",
        "reference": 19,
        "source": 12,
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
        "owner": 7,
        "presentationKey": "p:d70e244b956f7b9413e7ecfdc5acaf7f0c1a803f9c324701f4a93ba84bd4531d",
        "reference": 21,
        "source": 12,
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
        "owner": 6,
        "presentationKey": "p:357c832c3fdb19df21170c24f036255f3cfb6678764fe9aeac733bc9cabf8687",
        "reference": 20,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 9,
        "presentationKey": "p:21600d3949a97703baff896456a4fec2944039fc7ebfb039b46c8db567d6528d",
        "reference": 22,
        "source": 13,
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
              12
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "A",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:1acca06acc6aed21bdecc6ba51098a4e132f039b5fc0732b1fb08de1f9ece21b",
        "reference": 1,
        "source": 1,
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
        "owner": 11,
        "presentationKey": "p:8c9f5010f1ae116b87a6ea8c55fe52ea678d379705402df88706736e5aa0e5f4",
        "reference": 2,
        "source": 2,
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
              14,
              15
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "B",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:7a50be29cecfc8ee7968cb95d4c3006baa67cafa3069c7dead6c102cd074d957",
        "reference": 3,
        "source": 5,
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
        "name": "r1",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:8486a691c531e11240251757a6739943588dc19dca9d1f7798ab656c034d117d",
        "reference": 5,
        "source": 6,
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
        "name": "r2",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:dd3b5a0a72fa421d485247790d42caa926fa51dfbbe34d27ed1aaf26cbbcaa27",
        "reference": 6,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 13,
        "presentationKey": "p:0a83b698f573e08b00608e3b867d55f22707720e24fcef09330061dd013ae132",
        "reference": 14,
        "source": 8,
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
              18
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "C",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:134e0dbc4744898f4838b1d013fda390392d3479b91319235ccd793bd5838a00",
        "reference": 7,
        "source": 3,
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
        "owner": 17,
        "presentationKey": "p:4ac352b3aab5bdc9f84bd8f06a9b0b92f2ea435ca6ab514ec99b7165982bbcf6",
        "reference": 8,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 9,
        "provenance": "authored",
        "reference": 33,
        "source": 16,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 10,
        "provenance": "authored",
        "reference": 34,
        "source": 16,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 15,
        "provenance": "authored",
        "reference": 40,
        "source": 1,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 16,
        "provenance": "authored",
        "reference": 41,
        "source": 1,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 12,
        "provenance": "authored",
        "reference": 46,
        "source": 6,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 13,
        "provenance": "authored",
        "reference": 47,
        "source": 6,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "sequence",
      "lifelines": [
        11,
        17,
        13
      ],
      "messages": [
        {
          "label": "M1",
          "navigation": 14,
          "node": 1,
          "order": {
            "status": "resolved",
            "value": 1
          },
          "provenance": "authored",
          "source": {
            "lifeline": 11,
            "status": "resolved"
          },
          "target": {
            "lifeline": 13,
            "status": "resolved"
          }
        },
        {
          "label": "M2",
          "navigation": 11,
          "node": 6,
          "order": {
            "status": "resolved",
            "value": 2
          },
          "provenance": "authored",
          "source": {
            "lifeline": 17,
            "status": "resolved"
          },
          "target": {
            "lifeline": 13,
            "status": "resolved"
          }
        }
      ]
    }
  }
}

~~~
