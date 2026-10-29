# META
~~~ini
description=An action flow written with the then shorthand: each then continues from the nearest occurrence before it, and start, done, control nodes, signal actions and parameters are nodes of the flow
type=generate
libraries=standard
plugin=native:diagram
viewKind=action-flow-view
viewDocument=model.sysml
viewQualifiedName=Flow::process
~~~
# SOURCE
## model.sysml
~~~sysml
package Flow {
    private import StandardViewDefinitions::*;
    item def Order;
    attribute def Paid;
    action def Process {
        action prepare { out item order : Order; }
        action left { in item order : Order; }
        action right;
        action finish;
        flow prepare.order to left.order;
        first start;
        then prepare;
        then fork split;
        then left;
        then right;
        first left then joined;
        first right then joined;
        join joined;
        then decide choice;
        then action waitForPayment accept payment : Paid;
        then accept receipt : Paid;
        then finish;
        first finish then done;
    }
    view process : ActionFlowView { expose Process; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1b39d34db0be9f0abd7265027f8b3876ada97f0179ed62f796da43c34d00ed12") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "prepare::order")) (flowTarget (reference "left::order")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "prepare")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "left")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "right")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "left")) (succession (reference "joined")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "right")) (succession (reference "joined")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "Paid")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "finish")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "finish")) (succession (reference "done")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::choice"))) (kind decide) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined"))) (kind join) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Order")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Order")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::split"))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "Paid")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "process")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Process")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0))
      (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "joined")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "joined")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "prepare")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0))
      (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0))
      (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "Paid")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "prepare::order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "left::order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (kind featureTyping) (ordinal 0))
      (authored-target "Order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (kind featureTyping) (ordinal 0))
      (authored-target "Order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "Paid")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "process")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (kind acceptPayloadType) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (kind acceptPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "process")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "process")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::choice"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::decisions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::choice"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::joins"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::split"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::forks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::split"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "process")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)))))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::choice")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::DecisionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::decisions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::decisions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::DecisionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::JoinAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::joins"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::joins")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::JoinAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::split")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForkAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::forks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::forks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ForkAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::acceptSubactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::acceptActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "process")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 14) (end 10 19)) (probe (position 10 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 14) (end 15 18)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 14) (end 16 19)) (probe (position 16 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 14) (end 22 20)) (probe (position 22 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0) (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 24) (end 15 30)) (probe (position 15 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "joined")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 25) (end 16 31)) (probe (position 16 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "joined")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::joined")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 26) (end 22 30)) (probe (position 22 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 13) (end 11 20)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "prepare")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 13) (end 13 17)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 13) (end 14 18)) (probe (position 14 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0) (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::right")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 13) (end 21 19)) (probe (position 21 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind then-continuation) (ordinal 3))))) (kind thenTarget) (ordinal 0) (authored-target "finish")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::finish")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 30) (end 20 34)) (probe (position 20 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind accept-action) (ordinal 0))))) (kind acceptPayloadType) (ordinal 0) (authored-target "Paid")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 13) (end 9 26)) (probe (position 9 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "prepare::order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 30) (end 9 40)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Process")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "left::order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 38) (end 6 43)) (probe (position 6 38))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::left::order"))) (kind featureTyping) (ordinal 0) (authored-target "Order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 42) (end 5 47)) (probe (position 5 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::prepare::order"))) (kind featureTyping) (ordinal 0) (authored-target "Order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Order")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 52) (end 19 56)) (probe (position 19 52))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process::waitForPayment"))) (kind acceptPayloadType) (ordinal 0) (authored-target "Paid")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Paid")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 19) (end 24 33)) (probe (position 24 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::process"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 43) (end 24 50)) (probe (position 24 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "process")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Process")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:9c75b169a78d3783902323d1663d83109a603f0d5f2367cc26d274a16e72cae5",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/actions.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/occurrences.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        4,
        15,
        4,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        15,
        5,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        34,
        5,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        42,
        5,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        15,
        6,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        30,
        6,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        38,
        6,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        15,
        7,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        15,
        8,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        8,
        9,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        30,
        9,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        8,
        10,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        14,
        10,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        13,
        11,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        18,
        12,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        8,
        13,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        13,
        13,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        8,
        14,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        13,
        14,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        8,
        15,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        14,
        15,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        24,
        15,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        8,
        16,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        14,
        16,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        25,
        16,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        13,
        17,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        20,
        18,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        20,
        19,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        52,
        19,
        56
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        13,
        20,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        30,
        20,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        8,
        21,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        13,
        21,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        8,
        22,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        14,
        22,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        26,
        22,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        9,
        24,
        16
      ]
    },
    {
      "document": 1,
      "range": [
        42,
        9,
        42,
        14
      ]
    },
    {
      "document": 1,
      "range": [
        42,
        16,
        42,
        22
      ]
    },
    {
      "document": 1,
      "range": [
        42,
        27,
        42,
        36
      ]
    },
    {
      "document": 1,
      "range": [
        49,
        9,
        49,
        13
      ]
    },
    {
      "document": 1,
      "range": [
        49,
        15,
        49,
        21
      ]
    },
    {
      "document": 1,
      "range": [
        49,
        26,
        49,
        33
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Order"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Paid"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::choice"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::finish"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::joined"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::left"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::left::order"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::prepare"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::prepare::order"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::right"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::split"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Process::waitForPayment"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::process"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action::done"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action::start"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::endShot"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::startShot"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 31,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 17,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 21,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 33,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Process",
      "source": 35,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Flow::Process::",
      "source": 10,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "FlowEnd",
      "ownerQualifiedName": "Flow::Process::",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Flow::Process::::",
      "source": 10,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Flow::Process::::",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "acceptPayloadType",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "flowSource",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 71,
      "relationshipKind": "flowTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "thenTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "thenTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "thenTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "thenTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "succession",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "typing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "flow",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "succession",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "succession",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "succession",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "acceptPayloadType",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "succession",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "redefinition",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "typing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "redefinition",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "succession",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "succession",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "typing",
      "source": 18
    }
  ],
  "selectedView": {
    "reference": 15,
    "kind": "action-flow-view",
    "name": "process",
    "source": 38
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 1,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 21,
        "origin": 2,
        "provenance": "authored",
        "reference": 36,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 22,
        "origin": 2,
        "provenance": "implied",
        "reference": 78,
        "source": 18,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 3,
        "provenance": "authored",
        "reference": 37,
        "source": 0,
        "target": 3
      },
      {
        "kind": "succession",
        "navigation": 25,
        "origin": 3,
        "provenance": "implied",
        "reference": 83,
        "source": 20,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 35,
        "origin": 4,
        "provenance": "authored",
        "reference": 38,
        "source": 0,
        "target": 4
      },
      {
        "kind": "succession",
        "navigation": 36,
        "origin": 4,
        "provenance": "implied",
        "reference": 75,
        "source": 21,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 16,
        "provenance": "authored",
        "reference": 39,
        "source": 0,
        "target": 16
      },
      {
        "kind": "succession",
        "navigation": 16,
        "origin": 16,
        "provenance": "authored",
        "reference": 91,
        "source": 26,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 17,
        "provenance": "authored",
        "reference": 40,
        "source": 0,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 18,
        "provenance": "authored",
        "reference": 41,
        "source": 0,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 19,
        "provenance": "authored",
        "reference": 77,
        "source": 18,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 20,
        "provenance": "authored",
        "reference": 42,
        "source": 0,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 31,
        "origin": 5,
        "provenance": "authored",
        "reference": 43,
        "source": 0,
        "target": 5
      },
      {
        "kind": "succession",
        "navigation": 31,
        "origin": 5,
        "provenance": "authored",
        "reference": 87,
        "source": 6,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 21,
        "provenance": "authored",
        "reference": 44,
        "source": 0,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 28,
        "origin": 24,
        "provenance": "authored",
        "reference": 45,
        "source": 0,
        "target": 24
      },
      {
        "kind": "succession",
        "navigation": 28,
        "origin": 24,
        "provenance": "authored",
        "reference": 76,
        "source": 17,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 22,
        "provenance": "authored",
        "reference": 46,
        "source": 0,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 23,
        "provenance": "authored",
        "reference": 80,
        "source": 22,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 7,
        "provenance": "authored",
        "reference": 47,
        "source": 0,
        "target": 7
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 7,
        "provenance": "authored",
        "reference": 92,
        "source": 26,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 8,
        "provenance": "authored",
        "reference": 48,
        "source": 0,
        "target": 8
      },
      {
        "kind": "succession",
        "navigation": 17,
        "origin": 8,
        "provenance": "authored",
        "reference": 84,
        "source": 16,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 9,
        "provenance": "authored",
        "reference": 49,
        "source": 0,
        "target": 9
      },
      {
        "kind": "succession",
        "navigation": 19,
        "origin": 9,
        "provenance": "authored",
        "reference": 85,
        "source": 16,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 10,
        "provenance": "authored",
        "reference": 50,
        "source": 0,
        "target": 10
      },
      {
        "kind": "succession",
        "navigation": 33,
        "origin": 10,
        "provenance": "authored",
        "reference": 67,
        "source": 5,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 29,
        "origin": 6,
        "provenance": "authored",
        "reference": 51,
        "source": 0,
        "target": 6
      },
      {
        "kind": "succession",
        "navigation": 29,
        "origin": 6,
        "provenance": "authored",
        "reference": 74,
        "source": 24,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 42,
        "origin": 25,
        "provenance": "implied",
        "reference": 52,
        "source": 0,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 39,
        "origin": 26,
        "provenance": "implied",
        "reference": 53,
        "source": 0,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 11,
        "provenance": "authored",
        "reference": 54,
        "source": 0,
        "target": 11
      },
      {
        "kind": "flow",
        "navigation": 10,
        "origin": 11,
        "provenance": "implied",
        "reference": 81,
        "source": 23,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 12,
        "provenance": "authored",
        "reference": 56,
        "source": 11,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 13,
        "provenance": "authored",
        "reference": 72,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 14,
        "provenance": "authored",
        "reference": 57,
        "source": 11,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 15,
        "provenance": "authored",
        "reference": 73,
        "source": 14,
        "target": 15
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "action-flow-view",
    "metadata": {
      "actions": [
        0,
        5,
        6,
        18,
        20,
        21,
        22,
        25,
        26
      ],
      "controlNodes": [
        16,
        17,
        24
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              5,
              6,
              18,
              20,
              21,
              22
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              25,
              26
            ],
            "provenance": "inherited"
          },
          {
            "kind": "connections",
            "members": [
              11
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionDefinition",
        "multiplicity": null,
        "name": "Process",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:be54b9580f41edc5d8b22420b821750d6d5796aed3223e202ad95d0a3a5b8f8b",
        "reference": 2,
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
        "presentationKey": "p:3cdc67ffa1654379d66691426d3dfe4b134f68b83ce60ca5583fdb6f724c478c",
        "reference": 23,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:d5d5c21d9807506238a9642f81a57343dceb2aeb87e239ebda345c29186dcfd9",
        "reference": 27,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:b61a72b4ec0063a6f3553e5cc1010e6f9e151ed7b375b2175ea27f64ace0ecea",
        "reference": 28,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:e6e17c96c447f989bff9a3087b74ee3c70d0b4c135d529fcb0cae682aeefbc15",
        "reference": 30,
        "source": 35,
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
        "metaclass": "AcceptActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:330052481269521a268c6f9206066d5517f5d9eabf5034751265f08b8ffad53e",
        "reference": 21,
        "source": 31,
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
        "metaclass": "AcceptActionUsage",
        "multiplicity": null,
        "name": "waitForPayment",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:e63afe656e39d5c83877b03412e7d2fe8bc698a75af72e2550c2d18174930d96",
        "reference": 14,
        "source": 29,
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
        "presentationKey": "p:3627f6c388623461dd569b08aea71bc474af4e88a87eea1d65fda68c23e8efd3",
        "reference": 24,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:83205b187a03bd120fa084a59ac19c62d8b8a5960d64a345472f7a11dff2f2bf",
        "reference": 25,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:275fd8fbdaa2eae7858281091e44e6436281b4fe0099749b5c5a403e329edb7d",
        "reference": 26,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:33071b4a72176324c357cd54501f3efd6e54b0ddc344dd6cc27b90c33dd627f3",
        "reference": 29,
        "source": 33,
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
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:dabd977fb04e24e83e850e1d4621318ad882c741cabfc21cdad8518564af9fe9",
        "reference": 22,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 11,
        "presentationKey": "p:bcc2fe693a0b85e1ea0d712b259f1617ea03637e018d0fbb71b34577ff2960c1",
        "reference": 31,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 12,
        "presentationKey": "p:29478e89a6d5dc885e790599196c08150566863f959ded0cab4d97746081a47b",
        "reference": 33,
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
        "metaclass": "FlowEnd",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 11,
        "presentationKey": "p:d811ba93e7956eb220204e585fc189e56ae16f432ba7ad0d3a47694ea7bd9e1f",
        "reference": 32,
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
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 14,
        "presentationKey": "p:9aac023f239e17494278b42d8805d9c0cdb79ab9fef05becb1b6bcc93f9dafbe",
        "reference": 34,
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
        "metaclass": "ForkNode",
        "multiplicity": null,
        "name": "split",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:baf894c77acc3d817be471e68731bb7bcf19b87316fd477d0a00eca06049344d",
        "reference": 13,
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
        "metaclass": "JoinNode",
        "multiplicity": null,
        "name": "joined",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:34506476a89c33aab0a9c3f33b6416d9b3923772d5b2467ce9847e637eee772d",
        "reference": 7,
        "source": 27,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "items",
            "members": [
              19
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "left",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:dd7a67817ee841a73925446043a04826a7d08bafb2fe2bd31041f669bdd00f6b",
        "reference": 8,
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ItemUsage",
        "multiplicity": null,
        "name": "order",
        "notationRole": "usage",
        "owner": 18,
        "presentationKey": "p:efdb97da15751250bc1f13010071e7b4d6cca3f5e3d99d751a5d380669cfa235",
        "reference": 9,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Order",
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "right",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:4415900b0bb7b2aae6245a874001ac5af8ceccde829a6206e5ff4570b7df405b",
        "reference": 12,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "finish",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a123766bdc009602e04c19384b5899ac63c1039ce076a2ed74cf179daaf93866",
        "reference": 6,
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
            "kind": "items",
            "members": [
              23
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "prepare",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:43fb0dd77eef26604b7871a408cd81371e5d33f8e74488f98b2d72d8b5d282ed",
        "reference": 10,
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
        "direction": "out",
        "featureDirection": null,
        "metaclass": "ItemUsage",
        "multiplicity": null,
        "name": "order",
        "notationRole": "usage",
        "owner": 22,
        "presentationKey": "p:68f7313756a69f56745519d6ea416e8e61242fbc9c6f728280a66179fc170398",
        "reference": 11,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Order",
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
        "metaclass": "DecisionNode",
        "multiplicity": null,
        "name": "choice",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:3d72238ab8545f559d654b82198c94424048d4ee8d7ce36fc2c381193c684a0d",
        "reference": 5,
        "source": 28,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "done",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:8f2c0d09c7aa4b6a3fd3a24fde14f3d0bc39161c47034f008e8b263f9a69cf08",
        "reference": 17,
        "source": 42,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "start",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a67f6f29306944c4a08cd17fbc01af46bfbdf6d98fe47daf5f98665e37e02a59",
        "reference": 18,
        "source": 39,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 16
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 13,
        "provenance": "authored",
        "reference": 60,
        "source": 1,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 22,
        "provenance": "authored",
        "reference": 61,
        "source": 2,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 23,
        "provenance": "authored",
        "reference": 62,
        "source": 2,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 25,
        "provenance": "authored",
        "reference": 63,
        "source": 3,
        "target": {
          "node": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 26,
        "provenance": "authored",
        "reference": 64,
        "source": 3,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 36,
        "provenance": "authored",
        "reference": 65,
        "source": 4,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 37,
        "provenance": "authored",
        "reference": 66,
        "source": 4,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 79,
        "source": 19,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "acceptPayloadType",
        "navigation": 32,
        "provenance": "authored",
        "reference": 55,
        "source": 5,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 82,
        "source": 23,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 15,
        "provenance": "authored",
        "reference": 68,
        "source": 7,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 18,
        "provenance": "authored",
        "reference": 69,
        "source": 8,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 20,
        "provenance": "authored",
        "reference": 70,
        "source": 9,
        "target": {
          "node": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 34,
        "provenance": "authored",
        "reference": 71,
        "source": 10,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "acceptPayloadType",
        "navigation": 30,
        "provenance": "authored",
        "reference": 86,
        "source": 6,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 43,
        "provenance": "authored",
        "reference": 89,
        "source": 25,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 44,
        "provenance": "authored",
        "reference": 88,
        "source": 25,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 40,
        "provenance": "authored",
        "reference": 93,
        "source": 26,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 41,
        "provenance": "authored",
        "reference": 90,
        "source": 26,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "flowSource",
        "navigation": 10,
        "provenance": "authored",
        "reference": 58,
        "source": 11,
        "target": {
          "node": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "flowTarget",
        "navigation": 11,
        "provenance": "authored",
        "reference": 59,
        "source": 11,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "action-flow"
    }
  }
}

~~~
