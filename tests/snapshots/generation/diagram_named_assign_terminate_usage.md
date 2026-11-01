# META
~~~ini
description=An action usage retains a named assignment and `then terminate self` as a single terminal node with its succession (SysML 2.0 8.2.2.17.1, 8.2.2.17.5, 8.2.2.17.6)
type=generate
libraries=standard
plugin=native:diagram
viewKind=action-flow-view
viewDocument=model.sysml
viewQualifiedName=NamedUsage::flowView
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package NamedUsage {
    private import StandardViewDefinitions::*;
    private import ScalarValues::*;
    action run {
        attribute count : Integer := 0;
        first start;
        then action <inc> advance assign count := count + 1;
        then terminate self;
    }
    view flowView : ActionFlowView { expose run; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "NamedUsage::run::advance") (target "Actions::assignmentActions") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source (anonymous (owner "NamedUsage::run") (kind TerminateActionUsage) (ordinal 0))) (target "Actions::terminateActions") (provenance implied) (outcome resolved)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:dcb38eebec5f3e966c52817ac065ec9c7197279e375b7b1b0d4487831caeb397") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind view) (name "flowView")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "run")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminate-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (terminateTarget (reference "self")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (kind assign) (membership (kind feature) (visibility default)) (facts (short-name "inc")) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "count")) (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind assign) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind view) (name "flowView")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "run")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminateTarget) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (kind expressionOperand) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind view) (name "flowView")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind view) (name "flowView")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind terminateTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminateTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind view) (name "flowView")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::terminateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (state evaluated) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind view) (name "flowView")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TerminateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::terminateSubactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TerminateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::terminateActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::terminateSubactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TerminateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::terminateActions")) (scopes any feature))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance::onOccurrence")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind assign) (name "advance")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (outcome resolved) (operator "+" (feature-reference "count" (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count")))) (literal (value (kind integer) (integer 1)))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (named (kind attribute) (name "count")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 0))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 34)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 20) (end 9 34)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::flowView"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 44) (end 9 47)) (probe (position 9 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind view) (name "flowView")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "run")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 14) (end 5 19)) (probe (position 5 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 23) (end 7 27)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "NamedUsage")) (named (kind action) (name "run")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminateTarget) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 50) (end 6 55)) (probe (position 6 50))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (kind expressionOperand) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 41) (end 6 46)) (probe (position 6 41))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::advance"))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 26) (end 4 33)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "NamedUsage::run::count"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:460a87b5db12c71a5685798d6016758b2d4271a67a4e1a5e860d55aabd9b5415",
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
        11,
        3,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        18,
        4,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        26,
        4,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        8,
        5,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        14,
        5,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        26,
        6,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        41,
        6,
        46
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        50,
        6,
        55
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        23,
        7,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        9,
        9,
        17
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
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NamedUsage::flowView"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NamedUsage::run"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NamedUsage::run::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NamedUsage::run::advance"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NamedUsage::run::advance::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NamedUsage::run::advance::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "NamedUsage::run::count"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action::self"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action::start"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::startShot"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "ScalarValues::Integer"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "NamedUsage::run",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "TerminateActionUsage",
      "ownerQualifiedName": "NamedUsage::run",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "NamedUsage::run::advance",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "NamedUsage::run::advance::",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "NamedUsage::run::advance::::",
      "source": 6,
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
      "ordinal": 1,
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
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "terminateTarget",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "assignTarget",
      "source": 3
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
      "relationshipKind": "expressionOperand",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "redefinition",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "typing",
      "source": 9
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "action-flow-view",
    "name": "flowView",
    "source": 10
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 1,
        "provenance": "authored",
        "reference": 17,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 3,
        "provenance": "authored",
        "reference": 18,
        "source": 0,
        "target": 3
      },
      {
        "kind": "succession",
        "navigation": 5,
        "origin": 3,
        "provenance": "authored",
        "reference": 32,
        "source": 8,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 4,
        "provenance": "authored",
        "reference": 25,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 5,
        "provenance": "authored",
        "reference": 28,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 6,
        "provenance": "authored",
        "reference": 29,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 7,
        "provenance": "authored",
        "reference": 19,
        "source": 0,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 2,
        "provenance": "authored",
        "reference": 20,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 8,
        "origin": 2,
        "provenance": "authored",
        "reference": 27,
        "source": 3,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 8,
        "provenance": "implied",
        "reference": 21,
        "source": 0,
        "target": 8
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "action-flow-view",
    "metadata": {
      "actions": [
        0,
        2,
        3,
        8
      ],
      "controlNodes": []
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              7
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              2,
              3
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              8
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "run",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:01c42da5ade96f081cbaedcd78d2052e3bbf5fc3236d939ea6f26cc51319010e",
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
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:93729fd75dc267f4b908065ed25b40e95a0b4faf84ac16808867da27f2896943",
        "reference": 12,
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
        "metaclass": "TerminateActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d7c98e2107f28aff660f5677f1b615921abe9c66c4ebd18bb82e9bf01a571de6",
        "reference": 13,
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
            "kind": "attributes",
            "members": [
              4
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AssignmentActionUsage",
        "multiplicity": null,
        "name": "advance",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:5dcb34049638914293a9adac2ae44f7e89c64e4a02a7176e86ca1a46f18dec1b",
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 3,
        "presentationKey": "p:9442e5f3ee72ecd650bc6b9d77eacb1667692f232250f1ec0e1c793908493fb7",
        "reference": 14,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              6
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 4,
        "presentationKey": "p:caa36a94ac179df9834d72fb770d374f8e304be0c721cb7ac578c16d6e84a4c0",
        "reference": 15,
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
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 5,
        "presentationKey": "p:9a98588cf64c121c6bdd10a43ad6300c8988e38f7d2776852347f4a60d4bb5ec",
        "reference": 16,
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
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "count",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:0326ceb7d18bdc8ed769ae836cf3f92a2a185c2213f6e94189d9bc5d65eb4cbb",
        "reference": 6,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Integer",
              "reference": 11
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
        "presentationKey": "p:3bc7def1d065144d9fc3ff9e7409e1d2a124dadf50f3b0486bcb3f1801811bc5",
        "reference": 9,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 7
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 4,
        "provenance": "authored",
        "reference": 22,
        "source": 1,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "assignTarget",
        "navigation": 6,
        "provenance": "authored",
        "reference": 24,
        "source": 3,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "expressionOperand",
        "navigation": 7,
        "provenance": "authored",
        "reference": 26,
        "source": 3,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 30,
        "source": 7,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "terminateTarget",
        "navigation": 9,
        "provenance": "authored",
        "reference": 23,
        "source": 2,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 33,
        "source": 8,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 13,
        "provenance": "authored",
        "reference": 31,
        "source": 8,
        "target": {
          "reference": 10,
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
