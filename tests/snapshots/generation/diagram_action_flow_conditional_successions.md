# META
~~~ini
description=Conditional successions carry their guard: written in full with first, and in the then shorthand after a decide, where a brace-less if is a guarded succession and not an if action
type=generate
libraries=standard
plugin=native:diagram
viewKind=action-flow-view
viewDocument=model.sysml
viewQualifiedName=Flow::guarded
~~~
# SOURCE
## model.sysml
~~~sysml
package Flow {
    private import StandardViewDefinitions::*;
    private import ScalarValues::*;
    action def Guarded {
        in attribute ready : Boolean;
        in attribute urgent : Boolean;
        action check;
        action repair;
        action ship;
        action expedite;
        first start;
        then check;
        then decide outcome;
            if ready then ship;
            else repair;
        first repair then check;
        first ship if urgent then expedite;
        first ship if not urgent then done;
        first expedite then done;
    }
    view guarded : ActionFlowView { expose Guarded; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:47b5e8025aff5f337d754a86be9d9690ef1be7396278511cdf10c4d0fd962cab") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "check")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "ship")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "repair")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "repair")) (succession (reference "check")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "ship")) (succession (reference "expedite")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "ship")) (succession (reference "done")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "expedite")) (succession (reference "done")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "ready")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "urgent")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "urgent")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::outcome"))) (kind decide) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "guarded")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Guarded")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "repair")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0))
      (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 0))
      (authored-target "expedite")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "check")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "expedite")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 1))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "check")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0))
      (authored-target "repair")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "urgent")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "urgent")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "guarded")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Guarded")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 1)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "guarded")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "guarded")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::outcome"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::outcome"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::outcome"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::decisions"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::outcome"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::outcome"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "guarded")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::outcome")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "guarded")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (feature-reference "ready" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready")))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (feature-reference "urgent" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent")))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (operator "not" (feature-reference "urgent" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))))))
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
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 34)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 14) (end 10 19)) (probe (position 10 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 14) (end 15 20)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "repair")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 14) (end 16 18)) (probe (position 16 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 14) (end 17 18)) (probe (position 17 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 0) (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 14) (end 18 22)) (probe (position 18 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 0) (authored-target "expedite")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 26) (end 15 31)) (probe (position 15 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "check")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 34) (end 16 42)) (probe (position 16 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "expedite")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::expedite")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 38) (end 17 42)) (probe (position 17 38))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3))))) (kind succession) (ordinal 1) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 28) (end 18 32)) (probe (position 18 28))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 4))))) (kind succession) (ordinal 1) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 13) (end 11 18)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "check")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::check")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 26) (end 13 30)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ship")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 17) (end 14 23)) (probe (position 14 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 2))))) (kind thenTarget) (ordinal 0) (authored-target "repair")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::repair")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 15) (end 13 20)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind then-continuation) (ordinal 1)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 22) (end 16 28)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 2)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "urgent")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 26) (end 17 32)) (probe (position 17 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind action-def) (name "Guarded")) (anonymous (kind succession) (ordinal 3)) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "urgent")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 29) (end 4 36)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::ready"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 30) (end 5 37)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded::urgent"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 19) (end 20 33)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::guarded"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 43) (end 20 50)) (probe (position 20 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Flow")) (named (kind view) (name "guarded")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Guarded")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Flow::Guarded")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:a4d59f1142b9bb30236e1c1b6d8d070e3cd739b4e830b519bdf869fc4e142db6",
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
    },
    {
      "uri": "memory://snapshot/sysml.library/scalar_values.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        15,
        3,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        21,
        4,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        29,
        4,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        21,
        5,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        30,
        5,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        15,
        6,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        15,
        7,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        15,
        8,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        15,
        9,
        23
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
        19
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        13,
        11,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        20,
        12,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        21,
        13,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        26,
        13,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        17,
        14,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        17,
        14,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        8,
        15,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        14,
        15,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        26,
        15,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        8,
        16,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        14,
        16,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        34,
        16,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        8,
        17,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        14,
        17,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        38,
        17,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        8,
        18,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        14,
        18,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        28,
        18,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        9,
        20,
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
      "qualifiedName": "Flow::Guarded"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::check"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::expedite"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::outcome"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::ready"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::repair"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::ship"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::Guarded::urgent"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Flow::guarded"
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
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "ScalarValues::Boolean"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 17,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 18,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 21,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Flow::Guarded",
      "source": 27,
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
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 5,
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
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 14,
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
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "succession",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "thenTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "thenTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "thenTarget",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "redefinition",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "redefinition",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "succession",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "typing",
      "source": 12
    }
  ],
  "selectedView": {
    "reference": 9,
    "kind": "action-flow-view",
    "name": "guarded",
    "source": 30
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 1,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 18,
        "origin": 2,
        "provenance": "authored",
        "reference": 25,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 19,
        "origin": 2,
        "provenance": "implied",
        "reference": 57,
        "source": 11,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 21,
        "origin": 3,
        "provenance": "authored",
        "reference": 26,
        "source": 0,
        "target": 3
      },
      {
        "kind": "succession",
        "navigation": 22,
        "origin": 3,
        "provenance": "implied",
        "reference": 58,
        "source": 9,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 4,
        "provenance": "authored",
        "reference": 27,
        "source": 0,
        "target": 4
      },
      {
        "kind": "succession",
        "navigation": 25,
        "origin": 4,
        "provenance": "implied",
        "reference": 59,
        "source": 9,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 5,
        "provenance": "authored",
        "reference": 28,
        "source": 0,
        "target": 5
      },
      {
        "kind": "succession",
        "navigation": 28,
        "origin": 5,
        "provenance": "implied",
        "reference": 53,
        "source": 12,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 9,
        "provenance": "authored",
        "reference": 29,
        "source": 0,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 10,
        "provenance": "authored",
        "reference": 30,
        "source": 0,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 11,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 13,
        "provenance": "authored",
        "reference": 32,
        "source": 0,
        "target": 13
      },
      {
        "kind": "succession",
        "navigation": 13,
        "origin": 13,
        "provenance": "authored",
        "reference": 64,
        "source": 17,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 12,
        "provenance": "authored",
        "reference": 33,
        "source": 0,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 14,
        "provenance": "authored",
        "reference": 34,
        "source": 0,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 15,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 6,
        "provenance": "authored",
        "reference": 36,
        "source": 0,
        "target": 6
      },
      {
        "kind": "succession",
        "navigation": 11,
        "origin": 6,
        "provenance": "authored",
        "reference": 65,
        "source": 17,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 7,
        "provenance": "authored",
        "reference": 37,
        "source": 0,
        "target": 7
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 7,
        "provenance": "authored",
        "reference": 54,
        "source": 13,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 8,
        "provenance": "authored",
        "reference": 38,
        "source": 0,
        "target": 8
      },
      {
        "kind": "succession",
        "navigation": 17,
        "origin": 8,
        "provenance": "authored",
        "reference": 55,
        "source": 13,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 34,
        "origin": 16,
        "provenance": "implied",
        "reference": 39,
        "source": 0,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 31,
        "origin": 17,
        "provenance": "implied",
        "reference": 40,
        "source": 0,
        "target": 17
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "action-flow-view",
    "metadata": {
      "actions": [
        0,
        9,
        10,
        11,
        12,
        16,
        17
      ],
      "controlNodes": [
        13
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              14,
              15
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              9,
              10,
              11,
              12
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              16,
              17
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionDefinition",
        "multiplicity": null,
        "name": "Guarded",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:421a6240f6d475db4dbc8ec9af1f481b157bfad6d9bf0fa7513494b648fa44de",
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:a12f124a9f0f5a7186543902b690fbbbae5598cb1af1eda86df2c5ad1a0798ad",
        "reference": 16,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:1ceab3d9978292d068f4456c4a107c734267150c2835cd0cd98160573e7d45c2",
        "reference": 20,
        "source": 18,
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
        "presentationKey": "p:f3b6bb3ae2f4dfb46faa2109283870b98eafe64f674a6daf56a028fd89069135",
        "reference": 21,
        "source": 21,
        "stateSubaction": null,
        "successionGuard": "urgent",
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
        "presentationKey": "p:c5c99a914f5bdacf993996123045cee3937634c5515b8bc2ffa77a15b2634a2e",
        "reference": 22,
        "source": 24,
        "stateSubaction": null,
        "successionGuard": "not urgent",
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
        "presentationKey": "p:b7812a40c98f24037ae26a59d2ec76e30d9201cdb134804ceaa559e0c2df0371",
        "reference": 23,
        "source": 27,
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
        "presentationKey": "p:f2b11ae751404981464cef3d1cf14445ffafe9c4c49c63a421d1a076d75110f7",
        "reference": 17,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:92b894ef912a6832d09f5df08f734adf8421fcfdf5b14ba8102cf6af04b9b399",
        "reference": 18,
        "source": 14,
        "stateSubaction": null,
        "successionGuard": "ready",
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
        "presentationKey": "p:98810aaa020c312c11118a9073345ce83200de9c6f2bca0fb77718267c4656db",
        "reference": 19,
        "source": 17,
        "stateSubaction": null,
        "successionGuard": "else",
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
        "name": "ship",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:835045a5c7666bb550f3500832f2b3eaad24c5225141a46fe466f1d3fb585f59",
        "reference": 7,
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
        "name": "check",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:6838cf45c7c837bf32dd5607f20df05f1e51e81b98756f7c5f6a1c5104bd1fed",
        "reference": 2,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "repair",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d514dedce799021406d4b5f00b9fec6e9aa18be54b0ae06c8caff87cc2d1f86b",
        "reference": 6,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "expedite",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:8682f66ba05fcff316cb6cf7787096ec1325ae3051cd3a8c23f908bfe7128cbe",
        "reference": 3,
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
        "metaclass": "DecisionNode",
        "multiplicity": null,
        "name": "outcome",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:67374844f7a718cdf9ccbe1f3514f4cdc788f8679483fc4105b2063450535e8a",
        "reference": 4,
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "ready",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:0f6c14dee31c50516d1c6343847b86d9ca11adb45e42dc00a46d2537204bc6ed",
        "reference": 5,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 15
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "in",
        "featureDirection": null,
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "urgent",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:19c8bec36fadc3d7310e9ddd2a7722e1ac6ced877e182743f6c34040e7352bce",
        "reference": 8,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 15
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
        "name": "done",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:93aecf15c13413c2088bb4675474c42ecf85c2de5aac4e4efd854c306b2b193a",
        "reference": 11,
        "source": 34,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 10
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
        "presentationKey": "p:8e0b0487d24fb6fa1f43094a29727d6413bd0774f9839566774b146f83e6d4b0",
        "reference": 12,
        "source": 31,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 10
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 10,
        "provenance": "authored",
        "reference": 41,
        "source": 1,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 19,
        "provenance": "authored",
        "reference": 42,
        "source": 2,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 20,
        "provenance": "authored",
        "reference": 43,
        "source": 2,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 22,
        "provenance": "authored",
        "reference": 44,
        "source": 3,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 23,
        "provenance": "authored",
        "reference": 45,
        "source": 3,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 25,
        "provenance": "authored",
        "reference": 46,
        "source": 4,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 26,
        "provenance": "authored",
        "reference": 47,
        "source": 4,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 28,
        "provenance": "authored",
        "reference": 48,
        "source": 5,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 29,
        "provenance": "authored",
        "reference": 49,
        "source": 5,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 56,
        "source": 14,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 60,
        "source": 15,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 12,
        "provenance": "authored",
        "reference": 50,
        "source": 6,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 15,
        "provenance": "authored",
        "reference": 51,
        "source": 7,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 16,
        "provenance": "authored",
        "reference": 52,
        "source": 8,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 35,
        "provenance": "authored",
        "reference": 62,
        "source": 16,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 36,
        "provenance": "authored",
        "reference": 61,
        "source": 16,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 32,
        "provenance": "authored",
        "reference": 66,
        "source": 17,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 33,
        "provenance": "authored",
        "reference": 63,
        "source": 17,
        "target": {
          "reference": 14,
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
